pub trait Cursor: Clone {
    fn pos(&self) -> usize;
    fn goto(&mut self, pos: usize);
    fn advance(&mut self, len: usize);
    fn is_match(&self, pattern: &str) -> bool;
    fn take_match(&self, pattern: &str) -> &str;
}

#[derive(Clone)]
pub struct StrCursor<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> StrCursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }
}

impl<'a> Cursor for StrCursor<'a> {
    fn pos(&self) -> usize { self.pos }
    fn goto(&mut self, pos: usize) { self.pos = pos; }
    fn advance(&mut self, len: usize) { self.pos += len; }

    fn is_match(&self, pattern: &str) -> bool {
        self.input[self.pos..].starts_with(pattern)
    }

    fn take_match(&self, pattern: &str) -> &str {
        &self.input[self.pos..self.pos + pattern.len()]
    }
}
