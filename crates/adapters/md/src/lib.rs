use tome_core::{to_lines, TextAdapter};

/// Minimal markdown adapter (passthrough for now).
pub struct MdAdapter;

impl TextAdapter for MdAdapter {
    fn name(&self) -> &'static str {
        "md"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["md", "markdown"]
    }
    fn render_lines(&self, bytes: &[u8]) -> Vec<String> {
        // Future: real Markdown to plain text; for now treat as text.
        let s = String::from_utf8_lossy(bytes);
        to_lines(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handles_markdown_as_text() {
        let out = MdAdapter.render_lines(b"# Title\npara");
        assert_eq!(out, vec!["# Title", "para"]);
    }
}
