// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::contexts::{Cst, Ctx};
use std::fmt::Debug;

pub type ParseResult<C> = Result<(C, Cst), C>;

pub trait CanParse<C: Ctx>: Debug {
    fn parse(&self, ctx: C) -> ParseResult<C>;
}
