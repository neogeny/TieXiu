// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module provides a regex-crate-backed implementation of the pyre traits.

use crate::util::pyre::error::{Error, Result};
use crate::util::pyre::traits;
use regex::{Regex, RegexBuilder};

#[derive(Debug, Clone)]
pub struct Pattern {
    regex: Regex,
    anchored: Regex,
    pattern: String,
}

#[derive(Debug, Clone)]
pub struct Match<'a> {
    haystack: &'a str,
    // (start, end) byte offsets for each group
    groups: Vec<Option<(usize, usize)>>,
}

pub fn escape(pattern: &str) -> Box<str> {
    regex::escape(pattern).into()
}

pub fn compile(pattern: &str) -> Result<Pattern> {
    Pattern::new(pattern)
}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Self> {
        let regex = RegexBuilder::new(pattern).unicode(true).build()?;

        // Python's re.match only matches at the beginning of the string.
        let anchored_pattern = format!(r"\A(?:{})", pattern);
        let anchored = RegexBuilder::new(&anchored_pattern).unicode(true).build()?;

        Ok(Self {
            regex,
            anchored,
            pattern: pattern.to_string(),
        })
    }

    pub fn search<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        let caps = self.regex.captures(text)?;
        Some(Match::from_captures(text, &caps))
    }

    pub fn match_<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        let caps = self.anchored.captures(text)?;
        Some(Match::from_captures(text, &caps))
    }

    pub fn fullmatch<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        let m = self.match_(text)?;
        if m.end(None) == text.len() as isize {
            Some(m)
        } else {
            None
        }
    }

    pub fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String> {
        let maxsplit = maxsplit.unwrap_or(0);
        let mut result = Vec::new();
        let mut last_end = 0;

        for (splits_done, caps) in self.regex.captures_iter(text).enumerate() {
            if maxsplit > 0 && splits_done >= maxsplit {
                break;
            }
            let m = caps.get(0).unwrap(); // get(0) is always the whole match
            result.push(text[last_end..m.start()].to_string());

            // Add capturing groups to result
            for i in 1..caps.len() {
                if let Some(cap) = caps.get(i) {
                    result.push(text[cap.start()..cap.end()].to_string());
                } else {
                    result.push(String::new());
                }
            }
            last_end = m.end();
        }
        result.push(text[last_end..].to_string());
        result
    }

    pub fn findall(&self, text: &str) -> Vec<Vec<String>> {
        self.regex
            .captures_iter(text)
            .map(|caps| {
                if caps.len() == 1 {
                    let m = caps.get(0).unwrap();
                    vec![text[m.start()..m.end()].to_string()]
                } else {
                    (1..caps.len())
                        .map(|i| {
                            caps.get(i)
                                .map(|m| text[m.start()..m.end()].to_string())
                                .unwrap_or_default()
                        })
                        .collect()
                }
            })
            .collect()
    }

    pub fn finditer<'a>(&self, text: &'a str) -> Vec<Match<'a>> {
        self.regex
            .captures_iter(text)
            .map(|caps| Match::from_captures(text, &caps))
            .collect()
    }

    pub fn sub(&self, repl: &str, text: &str, count: Option<usize>) -> String {
        if let Some(c) = count {
            self.regex.replacen(text, c, repl).to_string()
        } else {
            self.regex.replace_all(text, repl).to_string()
        }
    }

    pub fn subn(&self, repl: &str, text: &str, count: Option<usize>) -> (String, usize) {
        let mut replacements = 0;
        let replaced = if let Some(c) = count {
            self.regex
                .replacen(text, c, |caps: &regex::Captures| {
                    replacements += 1;
                    let mut res = String::new();
                    caps.expand(repl, &mut res);
                    res
                })
                .to_string()
        } else {
            self.regex
                .replace_all(text, |caps: &regex::Captures| {
                    replacements += 1;
                    let mut res = String::new();
                    caps.expand(repl, &mut res);
                    res
                })
                .to_string()
        };
        (replaced, replacements)
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl<'a> Match<'a> {
    fn from_captures(haystack: &'a str, caps: &regex::Captures) -> Self {
        let mut groups = Vec::with_capacity(caps.len());
        for i in 0..caps.len() {
            groups.push(caps.get(i).map(|m| (m.start(), m.end())));
        }
        Self { haystack, groups }
    }

    pub fn group(&self, group: usize) -> Option<&'a str> {
        let (start, end) = self.groups.get(group)?.as_ref()?;
        Some(&self.haystack[*start..*end])
    }

    pub fn groups(&self) -> Vec<Option<&'a str>> {
        self.groups
            .iter()
            .map(|g| g.as_ref().map(|(s, e)| &self.haystack[*s..*e]))
            .collect()
    }

    pub fn start(&self, group: Option<usize>) -> isize {
        let group = group.unwrap_or(0);
        self.groups
            .get(group)
            .and_then(|g| g.as_ref())
            .map(|(s, _)| *s as isize)
            .unwrap_or(-1)
    }

    pub fn end(&self, group: Option<usize>) -> isize {
        let group = group.unwrap_or(0);
        self.groups
            .get(group)
            .and_then(|g| g.as_ref())
            .map(|(_, e)| *e as isize)
            .unwrap_or(-1)
    }

    pub fn span(&self, group: Option<usize>) -> (usize, usize) {
        let group = group.unwrap_or(0);
        self.groups
            .get(group)
            .and_then(|g| g.as_ref())
            .map(|(s, e)| (*s, *e))
            .unwrap_or((0, 0))
    }
}

impl<'a> traits::Match<'a> for Match<'a> {
    fn group(&self, group: usize) -> Option<&'a str> {
        self.group(group)
    }

    fn groups(&self) -> Vec<Option<&'a str>> {
        self.groups()
    }

    fn start(&self, group: Option<usize>) -> isize {
        self.start(group)
    }

    fn end(&self, group: Option<usize>) -> isize {
        self.end(group)
    }

    fn span(&self, group: Option<usize>) -> (usize, usize) {
        self.span(group)
    }
}

impl traits::Pattern for Pattern {
    type Match<'a>
        = Match<'a>
    where
        Self: 'a;
    type Error = Error;

    fn search<'a>(&self, text: &'a str) -> Option<Self::Match<'a>> {
        self.search(text)
    }

    fn match_<'a>(&self, text: &'a str) -> Option<Self::Match<'a>> {
        self.match_(text)
    }

    fn fullmatch<'a>(&self, text: &'a str) -> Option<Self::Match<'a>> {
        self.fullmatch(text)
    }

    fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String> {
        self.split(text, maxsplit)
    }

    fn findall(&self, text: &str) -> Vec<Vec<String>> {
        self.findall(text)
    }

    fn finditer<'a>(&self, text: &'a str) -> Vec<Self::Match<'a>> {
        self.finditer(text)
    }

    fn sub(&self, repl: &str, text: &str, count: Option<usize>) -> String {
        self.sub(repl, text, count)
    }

    fn subn(&self, repl: &str, text: &str, count: Option<usize>) -> (String, usize) {
        self.subn(repl, text, count)
    }

    fn pattern(&self) -> &str {
        self.pattern()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::pyre::traits::Match as TMatch;

    #[test]
    fn test_regex_basic() {
        let p = Pattern::new(r"\d+").unwrap();
        let m = p.search("abc123def").unwrap();
        assert_eq!(TMatch::group(&m, 0), Some("123"));
        assert_eq!(TMatch::start(&m, None), 3);
        assert_eq!(TMatch::end(&m, None), 6);
    }

    #[test]
    fn test_regex_anchored() {
        let p = Pattern::new(r"\d+").unwrap();
        assert!(p.match_("123abc").is_some());
        assert!(p.match_("abc123").is_none());
    }

    #[test]
    fn test_regex_groups() {
        let p = Pattern::new(r"(\d+)-(\w+)").unwrap();
        let m = p.search("123-abc").unwrap();
        assert_eq!(TMatch::group(&m, 1), Some("123"));
        assert_eq!(TMatch::group(&m, 2), Some("abc"));
    }

    #[test]
    fn test_regex_split() {
        let p = Pattern::new(r":").unwrap();
        let result = p.split("a:b:c", None);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_regex_findall() {
        let p = Pattern::new(r"\d").unwrap();
        let result = p.findall("1a2b3");
        assert_eq!(
            result,
            vec![
                vec!["1".to_string()],
                vec!["2".to_string()],
                vec!["3".to_string()]
            ]
        );
    }

    #[test]
    fn test_regex_sub() {
        let p = Pattern::new(r"\d").unwrap();
        let result = p.sub("X", "1a2b3", None);
        assert_eq!(result, "XaXbX");
    }

    #[test]
    fn test_regex_subn() {
        let p = Pattern::new(r"\d").unwrap();
        let (result, count) = p.subn("X", "1a2b3", None);
        assert_eq!(result, "XaXbX");
        assert_eq!(count, 3);
    }
}
