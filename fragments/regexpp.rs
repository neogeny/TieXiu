fn regexpp(pattern: &str) -> String {
    // Map control characters
    let mut result = String::with_capacity(pattern.len());
    for c in pattern.chars() {
        match c {
            '\n' => result.push_str(r"\n"),
            '\r' => result.push_str(r"\r"),
            '\t' => result.push_str(r"\t"),
            // ... add others from your ctrl_map
            _ => result.push(c),
        }
    }

    // Handle trailing backslashes for raw string safety
    let trailing_slashes = result.len() - result.trim_end_matches('\\').len();
    if trailing_slashes % 2 != 0 {
        result.push('\\');
    }

    // Choose delimiter based on quote counts
    let single_quotes = result.matches('\'').count();
    let double_quotes = result.matches('"').count();

    let (quote, escaped_quote) = if result.ends_with('\'') || single_quotes > double_quotes {
        ('"', "\\\"")
    } else {
        ('\'', "\\\'")
    };

    // Replace unescaped quotes (The "manual" negative lookbehind)
    let mut output = String::with_capacity(result.len() + 2);
    output.push('r');
    output.push(quote);

    let mut chars = result.chars().peekable();
    let mut was_backslash = false;

    while let Some(c) = chars.next() {
        if c == quote && !was_backslash {
            output.push_str(escaped_quote);
        } else {
            output.push(c);
        }
        // If current is backslash, and next is NOT a backslash, we set state
        // This handles \\" (escaped backslash, then unescaped quote) correctly
        was_backslash = (c == '\\') && !was_backslash;
    }

    output.push(quote);
    output
}
