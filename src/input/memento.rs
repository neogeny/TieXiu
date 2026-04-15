use console::style;
use std::fmt;

pub struct Memento {
    /// Absolute line number and content of captured lines, frozen in a boxed slice.
    window: Box<[(usize, Box<str>)]>,
    /// (Absolute Line, Absolute Column) - 1-indexed
    abs_start: (usize, usize),
    /// (Absolute Line, Absolute Column) - 1-indexed
    abs_mark: (usize, usize),
    /// The index within the `window` slice where the error starts and ends.
    _rel_start_idx: usize,
    rel_mark_idx: usize,
}

impl Memento {
    pub fn new(text: &str, start: usize, mark: usize) -> Self {
        // 1. Get absolute positions using platform-independent logic
        let abs_start = Self::pos_at(text, start);
        let abs_mark = Self::pos_at(text, mark);

        // 2. Determine the boundaries for our context window
        let mark_line_idx = abs_mark.0.saturating_sub(1);
        let start_line_idx = mark_line_idx.saturating_sub(4);

        // 3. Capture the lines into a Vec first to handle dynamic growth
        let window_vec: Vec<(usize, Box<str>)> = text
            .lines()
            .enumerate()
            .skip(start_line_idx)
            .take(mark_line_idx - start_line_idx + 1)
            .map(|(i, line)| (i + 1, Box::from(line)))
            .collect();

        // 4. Calculate relative indices for highlighting within the window
        // These tell us which element in our Boxed slice to annotate
        let rel_start_idx = abs_start.0.saturating_sub(start_line_idx + 1);
        let rel_mark_idx = abs_mark.0.saturating_sub(start_line_idx + 1);

        Self {
            window: window_vec.into_boxed_slice(), // Freeze into the final storage
            abs_start,
            abs_mark,
            _rel_start_idx: rel_start_idx,
            rel_mark_idx,
        }
    }

    /// Retrieve 1-indexed line and column positions
    fn pos_at(text: &str, mut mark: usize) -> (usize, usize) {
        mark = mark.min(text.len());
        let head = &text[0..mark];
        let line = head.lines().count();
        let col = head.lines().last().map_or(0, |l| l.chars().count());
        (line, col)
    }

    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_label = style("error").red().bold();
        let blue_pipe = style("|").blue().bold();
        let arrow = style("-->").blue().bold();

        // Header: error: description
        writeln!(f, "{}: syntax error detected", err_label)?;

        // Location: --> input:line:col
        writeln!(f, "  {} :{}:{}", arrow, self.abs_start.0, self.abs_start.1)?;
        writeln!(f,1, "   {}", blue_pipe)?;

        for (idx, (num, content)) in self.window.iter().enumerate() {
            // Print the line number and code content
            writeln!(
                f,
                "{:>2} {} {}",
                style(num).blue().bold(),
                blue_pipe,
                content
            )?;

            // If this line is the "mark" point, draw the caret
            if idx == self.rel_mark_idx {
                let padding = " ".repeat(self.abs_mark.1.saturating_sub(1));
                writeln!(f, "   {} {}{}", blue_pipe, padding, style("^").red().bold())?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Memento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(f)
    }
}

impl fmt::Debug for Memento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(f)
    }
}
