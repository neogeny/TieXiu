use super::Cursor;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct StrCursor<'a> {
    text: &'a str,
    offset: usize,

    whitespace_pattern: &'a str,
    eol_comments_pattern: &'a str,
    comments_pattern: &'a str,
}

impl<'a> StrCursor<'a> {
    pub fn new(
        text: &'a str,
        offset: usize,
        whitespace_pattern: &'a str,
        eol_comments_pattern: &'a str,
        comments_pattern: &'a str,
    ) -> Self {
        Self {
            text,
            offset,
            whitespace_pattern,
            eol_comments_pattern,
            comments_pattern,
        }
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

    fn pattern(&mut self, pattern: &str) -> Option<&'a str> {
        let re = Regex::new(pattern).ok()?;
        let caps = re.captures_at(self.text, self.offset)?;

        // Only match if it starts EXACTLY at the cursor
        let whole = caps.get(0)?;
        if whole.start() != self.offset {
            return None;
        }

        // Move the cursor by the whole match, but return the "Value"
        self.offset = whole.end();

        // Logic: If there is 1+ group, return group 1. Else return group 0.
        Some(caps.get(1).or(caps.get(0))?.as_str())
    }

    fn next_token(&mut self) {
        let mut last_p = usize::MAX;

        // Keep eating as long as we are making progress
        while self.offset != last_p {
            last_p = self.offset;

            self.eat_pattern(self.whitespace_pattern);
            if self.eat_pattern(self.eol_comments_pattern) {
                self.eat_pattern(self.whitespace_pattern);
            }
            self.eat_pattern(self.comments_pattern);
        }
    }

    fn eat_pattern(&mut self, pattern: &str) -> bool {
        if pattern.is_empty() {
            return false;
        }

        match Regex::new(pattern) {
            Err(_) => return false,
            Ok(re) => {
                if let Some(mat) = re.find_at(self.text, self.offset) {
                    if mat.start() != self.offset {
                        return false;
                    }
                    self.offset = mat.end();
                    return true;
                }
            }
        }
        false
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
