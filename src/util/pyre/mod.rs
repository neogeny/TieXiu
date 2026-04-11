mod fancy;
mod traits;

pub use fancy::*;

pub fn escape(pattern: &str) -> Box<str> {
    fancy::escape(pattern)
}

pub fn compile(pattern: &str) -> Result<Pattern, Error> {
    Pattern::new(pattern)
}

pub fn searchi<'a>(pattern: &str, text: &'a str) -> Option<Match<'a>> {
    Pattern::new(pattern).ok()?.search(text)
}

pub fn match_<'a>(pattern: &str, text: &'a str) -> Option<Match<'a>> {
    Pattern::new(pattern).ok()?.match_(text)
}

pub fn fullmatch<'a>(pattern: &str, text: &'a str) -> Option<Match<'a>> {
    Pattern::new(pattern).ok()?.fullmatch(text)
}

pub fn split(pattern: &str, text: &str, maxsplit: Option<usize>) -> Vec<String> {
    match Pattern::new(pattern) {
        Ok(p) => p.split(text, maxsplit),
        Err(_) => vec![text.to_string()],
    }
}

pub fn findall(pattern: &str, text: &str) -> Vec<String> {
    match Pattern::new(pattern) {
        Ok(p) => p.findall(text),
        Err(_) => vec![],
    }
}

pub fn finditer<'a>(pattern: &str, text: &'a str) -> Vec<Match<'a>> {
    match Pattern::new(pattern) {
        Ok(p) => p.finditer(text),
        Err(_) => vec![],
    }
}

pub fn sub(pattern: &str, repl: &str, text: &str, count: Option<usize>) -> String {
    match Pattern::new(pattern) {
        Ok(p) => p.sub(repl, text, count),
        Err(_) => text.to_string(),
    }
}

pub fn subn(pattern: &str, repl: &str, text: &str, count: Option<usize>) -> (String, usize) {
    match Pattern::new(pattern) {
        Ok(p) => p.subn(repl, text, count),
        Err(_) => (text.to_string(), 0),
    }
}

pub fn purge() {}