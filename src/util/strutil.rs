/// Detects a blank line: a sequence of at least two line breaks
/// with only whitespace in between. Returns the byte offset to
/// the start of the next line with content.
pub fn blank_line(text: &str) -> Option<usize> {
    let base_ptr = text.as_ptr() as usize;
    let mut lines = text.lines().peekable();

    // 1. First line MUST be empty/whitespace
    let first = lines.next()?;
    if !first.trim().is_empty() {
        return None;
    }

    // 2. Second line MUST also be empty/whitespace to constitute a "blank line"
    let second = lines.next()?;
    if !second.trim().is_empty() {
        return None;
    }

    // 3. Success! Return offset to the start of the THIRD line (the next content)
    match lines.peek() {
        Some(third) => Some(third.as_ptr() as usize - base_ptr),
        None => Some(text.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_newline_is_none() {
        // A single newline is a line break, not a "blank line"
        assert_eq!(blank_line(" \r\nrule"), None);
    }

    #[test]
    fn test_valid_blank_line() {
        // Two line breaks with whitespace between = Blank Line
        // Returns offset to "rule"
        assert_eq!(blank_line("  \n  \nrule"), Some(6));
    }

    #[test]
    fn test_not_blank_line() {
        // First line has content
        assert_eq!(blank_line("content\n\n"), None);
    }

    #[test]
    fn test_empty_string() {
        // No lines to even check
        assert_eq!(blank_line(""), None);
    }

    #[test]
    fn test_trailing_blank_line() {
        // Two empty lines at the end of a string
        assert_eq!(blank_line(" \n \n"), Some(4));
    }

    #[test]
    fn test_blank_line_and_end() {
        // Two empty lines at the end of a string
        assert_eq!(blank_line(" \n "), Some(3));
    }

    #[test]
    fn test_multiple_blank_lines() {
        // Two empty lines at the end of a string
        assert_eq!(blank_line("  \n\n\n"), Some(4));
    }
}
