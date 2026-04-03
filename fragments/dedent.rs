fn dedent(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    // Find the common minimum indentation of non-empty lines
    let min_indent = lines.iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);

    lines.iter()
        .map(|l| if l.len() > min_indent { &l[min_indent..] } else { l.trim() })
        .collect::<Vec<_>>()
        .join("\n")
}
