use console::style;
use std::fmt;

pub struct Memento {
    /// Absolute line number and content of captured lines.
    window: Box<[(usize, Box<str>)]>,
    /// (Line number, column index) for the error start.
    start_pos: (usize, usize),
    /// (Line number, column index) for the error mark (end).
    mark_pos: (usize, usize),
}

impl Memento {
    pub fn new(text: &str, start: usize, mark: usize) -> Self {
        // Calculate positions using platform-independent .lines() logic
        let start_pos = Self::pos_at(text, start);
        let mark_pos = Self::pos_at(text, mark);

        // Calculate the window: up to 4 lines before the mark line
        let mark_line_idx = mark_pos.0.saturating_sub(1);
        let start_line_idx = mark_line_idx.saturating_sub(4);

        // Capture relevant lines into the window
        let window = text.lines()
            .enumerate()
            .skip(start_line_idx)
            .take(mark_line_idx - start_line_idx + 1)
            .map(|(i, line)| (i + 1, Box::from(line)))
            .collect::<Vec<_>>().into_boxed_slice();

        Self {
            window,
            start_pos,
            mark_pos,
        }
    }

    /// Platform-independent position retrieval
    fn pos_at(text: &str, mut mark: usize) -> (usize, usize) {
        mark = mark.min(text.len());
        let head = &text[0..mark];
        let line = head.lines().count();
        let col = head.lines().last().map_or(0, |l| l.chars().count());
        (line, col)
    }

    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_color = style("error").red().bold();
        let pipe = style("|").blue().bold();

        // Header and Location
        writeln!(f, "{}: invalid token found", error_color)?;
        writeln!(f, "  {} {}:{}:{}",
            style("-->").blue().bold(),
            "input",
            self.start_pos.0,
            self.start_pos.1
        )?;

        writeln!(f, "   {}", pipe)?;

        for (num, content) in &self.window {
            let is_mark_line = *num == self.mark_pos.0;

            // Print line number and text
            write!(f, "{:>2} {} ", style(num).blue().bold(), pipe)?;
            writeln!(f, "{}", content)?;

            // If this is the line where the error ends, draw the caret
            if is_mark_line {
                // Use chars().count() for correct visual alignment with Unicode
                let padding = " ".repeat(self.mark_pos.1.saturating_sub(1));
                writeln!(f, "   {} {}{}",
                    pipe,
                    padding,
                    style("^").red().bold()
                )?;
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