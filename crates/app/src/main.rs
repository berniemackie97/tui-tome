use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::{
    fs,
    io::{stdout, Stdout},
    path::{Path, PathBuf},
    time::Duration,
};
use tome_adapter_md::MdAdapter;
use tome_adapter_txt::TxtAdapter;
use tome_core::TextAdapter;

#[derive(Parser, Debug)]
#[command(name = "Tome", version, about = "TUI reader/editor (q/Esc to quit)")]
struct Args {
    /// Path to a file to open (.txt, .md for now)
    path: PathBuf,
}

struct App {
    file: String,
    lines: Vec<String>,
    scroll: usize,
}

impl App {
    fn new(file: String, lines: Vec<String>) -> Self {
        Self { file, lines, scroll: 0 }
    }
    fn max_scroll(&self, height: usize) -> usize {
        self.lines.len().saturating_sub(height)
    }
    fn clamp_scroll(&mut self, height: usize) {
        let max = self.max_scroll(height);
        if self.scroll > max {
            self.scroll = max;
        }
    }
}

fn adapters() -> Vec<Box<dyn TextAdapter>> {
    vec![Box::new(TxtAdapter), Box::new(MdAdapter)]
}

fn pick_adapter<'a>(set: &'a [Box<dyn TextAdapter>], path: &Path) -> &'a dyn TextAdapter {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        let ext = ext.to_ascii_lowercase();
        for a in set {
            if a.extensions().iter().any(|e| e.eq_ignore_ascii_case(&ext)) {
                return a.as_ref();
            }
        }
    }
    // default to txt adapter
    set[0].as_ref()
}

fn load_lines(path: &Path) -> Result<(String, Vec<String>)> {
    let bytes = fs::read(path).with_context(|| format!("reading {:?}", path))?;
    let file = path.file_name().and_then(|s| s.to_str()).unwrap_or("untitled").to_string();
    let set = adapters();
    let adapter = pick_adapter(&set, path);
    let lines = adapter.render_lines(&bytes);
    Ok((file, lines))
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.size());

    let height = chunks[0].height as usize;
    let total = app.lines.len();
    let start = app.scroll.min(total);
    let end = (start + height).min(total);
    let body = if start < end { app.lines[start..end].join("\n") } else { String::new() };

    let title = format!("Tome — {} (q/Esc to quit)", app.file);
    let para = Paragraph::new(body).block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(para, chunks[0]);

    let pct =
        if total == 0 { 0u32 } else { ((start as f32 / total as f32) * 100.0).round() as u32 };
    let status = format!("L{}–{} / {}  ·  {}%", start.saturating_add(1), end, total, pct);
    let sb = Paragraph::new(status).block(Block::default().borders(Borders::TOP));
    frame.render_widget(sb, chunks[1]);
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (file, lines) = load_lines(&args.path)?;
    // setup terminal
    enable_raw_mode()?;
    let mut stdout: Stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(file, lines);

    loop {
        terminal.draw(|f| ui(f, &app))?;
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(k) => match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up => app.scroll = app.scroll.saturating_sub(1),
                    KeyCode::Down => app.scroll = app.scroll.saturating_add(1),
                    KeyCode::PageUp => app.scroll = app.scroll.saturating_sub(10),
                    KeyCode::PageDown => app.scroll = app.scroll.saturating_add(10),
                    KeyCode::Home | KeyCode::Char('g') => app.scroll = 0,
                    KeyCode::End | KeyCode::Char('G') => {
                        let size = terminal.size()?;
                        app.scroll = app.max_scroll(size.height as usize);
                    }
                    _ => {}
                },
                Event::Resize(_, _) => {
                    let size = terminal.size()?;
                    app.clamp_scroll(size.height as usize);
                }
                _ => {}
            }
            let size = terminal.size()?;
            let height = size.height as usize;
            let max = app.max_scroll(height);
            if app.scroll > max {
                app.scroll = max;
            }
        }
    }

    // teardown
    disable_raw_mode()?;
    let out = terminal.backend_mut();
    execute!(out, LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
