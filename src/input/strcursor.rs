use super::Cursor;

#[derive(Debug, Clone, Copy)]
pub struct StrCursor<'a> {
    text: &'a str,
    offset: usize,
}

impl<'a> StrCursor<'a> {
    pub fn new(text: &'a str, offset: usize) -> Self {
        Self { text, offset }
    }
}

impl<'a> Cursor for StrCursor<'a> {
    fn mark(&self) -> usize {
        self.offset
    }

    fn reset(&mut self, mark: usize) {
        self.offset = mark;
    }

    fn textstr(&self) -> &str {
        self.text
    }

    fn token(&mut self, token: &str) -> bool {
        if self.text[self.offset..].starts_with(token) {
            self.offset += token.len();
            true
        } else {
            false
        }
    }
}
// #[inline]
// fn pos(&self) -> usize {
//     self.offset
// }
//
// #[inline]
// fn set_pos(&mut self, pos: usize) {
//     self.offset = pos;
// }
//
// fn peek(&self, len: usize) -> Option<&str> {
//     self.buffer.get(self.offset..self.offset + len)
// }
//
// #[inline]
// fn is_at_end(&self) -> bool {
//     self.offset >= self.buffer.len()
// }
//
// fn remaining(&self) -> &str {
//     &self.buffer[self.offset..]
// }
