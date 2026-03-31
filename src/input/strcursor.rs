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

    fn at_end(&self) -> bool {
        self.offset >= self.text.len()
    }

    fn next(&mut self) -> Option<char> {
        if self.at_end() {
            return None;
        }
        // Slice from the current byte offset to the end
        let rest = self.text.get(self.offset..)?;
        // Decode the first Unicode character
        let c = rest.chars().next()?;
        // Increment the byte offset by the character's UTF-8 width
        self.offset += c.len_utf8();

        Some(c)
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
