use tome_core::{to_lines, TextAdapter};

pub struct TxtAdapter;

impl TextAdapter for TxtAdapter {
    fn name(&self) -> &'static str {
        "txt"
    }
    fn extensions(&self) -> &'static [&'static str] {
        &["txt", "text", "log"]
    }
    fn render_lines(&self, bytes: &[u8]) -> Vec<String> {
        let s = String::from_utf8_lossy(bytes);
        to_lines(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn splits_lines() {
        let out = TxtAdapter.render_lines(b"a\r\nb\nc");
        assert_eq!(out, vec!["a", "b", "c"]);
    }
}
