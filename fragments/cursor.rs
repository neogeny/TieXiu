use regex::Regex;

struct Cursor<'a> {
    rest: &'a str,
}

impl<'a> Cursor<'a> {

    pub fn match_token(&mut self, token: &str) -> bool {
        if self.rest.starts_with(token) {
            // "Advance" the iterator by slicing
            self.rest = &self.rest[token.len()..];
            true
        } else {
            false
        }
    }

    pub fn match_re(&mut self, re: &Regex) -> Option<&'a str> {
        // find() returns the first match.
        // We use a regex that starts with ^ to force a match at the current position.
        if let Some(m) = re.find(self.rest) {
            if m.start() == 0 {
                let matched_text = m.as_str();
                // Advance the "iterator" by slicing past the match
                self.rest = &self.rest[m.end()..];
                return Some(matched_text);
            }
        }
        None
    }
}
