use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Normalize line endings so adapters can compare content consistently.
pub fn normalize_eol(s: &str) -> String {
    s.replace("\r\n", "\n")
}

/// Byte offset within a UTF-8 buffer (half-open ranges below are byte-based).
pub type BytePos = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    pub start: BytePos,
    pub end: BytePos, // half-open [start, end)
}
impl Range {
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DocumentId(pub Uuid);

impl DocumentId {
    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }
}

pub const CONTEXT: usize = 48;

/// Stable-ish pointer to a selection using surrounding context (doesn’t touch source file).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Anchor {
    pub before: String,
    pub target: String,
    pub after: String,
}

impl Anchor {
    /// Create an anchor from `text` and a byte `range` (clamped to bounds).
    pub fn create(text: &str, range: Range) -> Self {
        let text = normalize_eol(text);
        let start = range.start.min(text.len());
        let end = range.end.min(text.len()).max(start);
        let b0 = start.saturating_sub(CONTEXT);
        let a1 = (end + CONTEXT).min(text.len());
        Self {
            before: text[b0..start].to_string(),
            target: text[start..end].to_string(),
            after: text[end..a1].to_string(),
        }
    }

    /// Resolve this anchor in a possibly edited `text` via best context match.
    pub fn resolve(&self, text: &str) -> Option<Range> {
        if self.target.is_empty() {
            return None;
        }
        let text = normalize_eol(text);
        let mut idx = 0usize;
        let mut fallback: Option<Range> = None;

        while let Some(found) = text[idx..].find(&self.target) {
            let start = idx + found;
            let end = start + self.target.len();

            let before_start = start.saturating_sub(self.before.len());
            let before_ok = text[before_start..start].ends_with(&self.before);

            let after_end = (end + self.after.len()).min(text.len());
            let after_ok = text[end..after_end].starts_with(&self.after);

            if before_ok && after_ok {
                return Some(Range { start, end });
            }
            if fallback.is_none() {
                fallback = Some(Range { start, end });
            }
            idx = end;
        }
        fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalizes_windows_crlf_to_lf() {
        let input = "a\r\nb\r\nc";
        assert_eq!(normalize_eol(input), "a\nb\nc");
    }
    #[test]
    fn anchor_resolves_after_inserting_line_above() {
        let text = "alpha\nbeta\ngamma\n";
        let start = text.find("beta").unwrap();
        let end = start + "beta".len();
        let a = Anchor::create(text, Range { start, end });

        let changed = "alpha\nNEW\nbeta\ngamma\n";
        let resolved = a.resolve(changed).expect("should resolve");
        assert_eq!(&changed[resolved.start..resolved.end], "beta");
    }
    #[test]
    fn document_id_creates_uuid() {
        let a = DocumentId::random();
        let b = DocumentId::random();
        assert_ne!(a.0, b.0);
    }
    #[test]
    fn document_id_serde_roundtrip() {
        let id = DocumentId::random();
        let s = serde_json::to_string(&id).unwrap();
        let back: DocumentId = serde_json::from_str(&s).unwrap();
        assert_eq!(id.0, back.0);
    }
}
