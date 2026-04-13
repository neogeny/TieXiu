// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

pub fn get_line_col(text: &str, offset: usize) -> (usize, usize) {
    let head = &text[..offset];
    let lines: Vec<&str> = head.split_terminator('\n').collect();

    let line = lines.len();
    let col = lines.last().map_or(1, |last| last.chars().count() + 1);

    (line, col)
}

// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

pub fn get_line_col(text: &str, offset: usize) -> (usize, usize) {
    // Slice the text up to the byte offset
    let head = &text[..offset];

    // The number of line breaks in the head gives us the current line index
    // We add 1 because it's 1-based indexing.
    let line = head.lines().count();

    // The column is the character count of the remaining content after the last newline
    // If the offset is on a new line, lines().last() handles it,
    // but we need to account for trailing newlines.
    let col = head.lines().last().map_or(1, |l| l.chars().count() + 1);

    // Edge case: if the head ends with a newline, .lines() ignores it.
    // We adjust to line + 1, column 1.
    if head.ends_with('\n') {
        (line + 1, 1)
    } else {
        (line, col)
    }
}
