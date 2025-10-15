/// Normalize line endings so adapters can compare content consistently.
pub fn normalize_eol(s: &str) -> String {
    s.replace("\r\n", "\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalizes_windows_crlf_to_lf() {
        let input = "a\r\nb\r\nc";
        assert_eq!(normalize_eol(input), "a\nb\nc");
    }
}
