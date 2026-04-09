// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use fancy_regex::Error;
pub use fancy_regex::{Captures, Match, Regex};

pub fn new(pattern: &str) -> Result<Regex, Error> {
    Regex::new(pattern)
}

pub fn compile(pattern: &str) -> Regex {
    Regex::new(pattern).unwrap()
}
