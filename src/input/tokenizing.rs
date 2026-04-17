// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cfg::constants::*;
use crate::input::Error;
use crate::util::pyre::Pattern;

#[derive(Clone, Debug)]
pub struct TokenizingPatterns {
    pub wsp: Pattern,
    pub cmt: Pattern,
    pub eol: Pattern,
}

impl TokenizingPatterns {
    pub const DEFAULT_WSP: &'static str = r"\s+";
    pub const DEFAULT_EOL: &'static str = r"//.*$";
    pub const DEFAULT_CMT: &'static str = r"(?ms)/\*.*\*/";

    pub fn compile(kind: &'static str, pattern: &str) -> Result<Pattern, Error> {
        let p = Pattern::new(pattern).map_err(|source| Error::InvalidRegex {
            kind,
            pattern: pattern.to_string(),
            source,
        })?;
        Self::validate_no_empty_match(&p, kind);
        Ok(p)
    }

    pub fn validate_no_empty_match(pattern: &Pattern, kind: &str) {
        assert!(
            pattern.search("").is_none(),
            "pattern '{}' for {} matches empty string, which would cause infinite loop",
            pattern.pattern(),
            kind
        );
    }

    pub fn try_new(ws: &str, cmt: &str, eol: &str) -> Result<Self, Error> {
        Ok(Self {
            wsp: Self::compile(WSP, ws)?,
            cmt: Self::compile(CMT, cmt)?,
            eol: Self::compile(EOL, eol)?,
        })
    }
}

impl Default for TokenizingPatterns {
    fn default() -> Self {
        Self::try_new(Self::DEFAULT_WSP, Self::DEFAULT_CMT, Self::DEFAULT_EOL)
            .expect("default StrCursor regex patterns must be valid")
    }
}
