// Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::cst::Cst;
use super::ast::Ast;

/// The final outcome of a parsing rule.
/// 
/// This acts as a sovereign container for all "Iron" cargo. 
/// It remains opaque during the parsing process to be resolved 
/// only at the boundary of a finished parse.
pub enum Parsed {
    /// A fast-path for raw string matches.
    Token(String),
    /// A fragment of the concrete syntax tree.
    Cst(Cst),
    /// A structured mapping of named elements.
    Ast(Ast),
}

impl From<String> for Parsed {
    fn from(s: String) -> Self {
        Parsed::Token(s)
    }
}

impl From<Cst> for Parsed {
    fn from(c: Cst) -> Self {
        Parsed::Cst(c)
    }
}

impl From<Ast> for Parsed {
    fn from(a: Ast) -> Self {
        Parsed::Ast(a)
    }
}

impl Parsed {
    // Methods for JSON serialization and Python boundary 
    // resolution will be added here once the State logic is settled.
}