// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Debug;
use crate::contexts::{Cst, Ctx};

pub type ParseResult<C> = Result<(C, Cst), C>;

pub trait CanParse<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}


