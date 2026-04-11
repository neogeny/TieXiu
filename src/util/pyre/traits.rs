// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Traits that define the pyre Pattern and Match interfaces.
//! These traits mirror the Python `re` module API.

pub trait Pattern: Clone {
    type Error: std::error::Error;

    fn new(pattern: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn search<'a, M: Match<'a>>(&self, text: &'a str) -> Option<M>;

    fn match_<'a, M: Match<'a>>(&self, text: &'a str) -> Option<M>;

    fn fullmatch<'a, M: Match<'a>>(&self, text: &'a str) -> Option<M>;

    fn split(&self, text: &str, maxsplit: Option<usize>) -> Vec<String>;

    fn findall(&self, text: &str) -> Vec<String>;

    fn finditer<'a, M: Match<'a>>(&self, text: &'a str) -> Vec<M>;

    fn sub(&self, repl: &str, text: &str, count: Option<usize>) -> String;

    fn subn(&self, repl: &str, text: &str, count: Option<usize>) -> (String, usize);

    fn pattern(&self) -> &str;
}

pub trait Match<'a> {
    fn group(&self, group: usize) -> Option<&'a str>;

    fn groups(&self) -> Vec<Option<&'a str>>;

    fn start(&self, group: Option<usize>) -> isize;

    fn end(&self, group: Option<usize>) -> isize;

    fn span(&self, group: Option<usize>) -> (usize, usize);
}
