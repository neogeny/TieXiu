use std::fmt;

pub struct PrettyPrinter<'a> {
    f: &'a mut fmt::Formatter<'a>,
    indent_level: usize,
    width: usize,
}

impl<'a> PrettyPrinter<'a> {
    pub fn new(f: &'a mut fmt::Formatter<'a>) -> Self {
        Self { f, indent_level: 0, width: 88 }
    }

    /// Your contextmanager 'with self.indent():'
    pub fn indented<F>(&mut self, f: F) -> fmt::Result
    where F: FnOnce(&mut Self) -> fmt::Result
    {
        self.indent_level += 4;
        let res = f(self);
        self.indent_level -= 4;
        res
    }

    pub fn print(&mut self, text: &str) -> fmt::Result {
        let spaces = " ".repeat(self.indent_level);
        write!(self.f, "{}{}", spaces, text)
    }
}
