use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Stdout};
use std::time::Duration;

fn ui(frame: &mut Frame) {
    let block = Block::default()
        .title("Tome — TUI Reader (q to quit)")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(block, frame.size());
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout: Stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(ui)?; // <— no redundant closure
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(k) = event::read()? {
                if matches!(k.code, KeyCode::Char('q') | KeyCode::Esc) {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    let backend = terminal.backend_mut();
    execute!(backend, LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
