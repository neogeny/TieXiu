// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Debug;
use super::Cursor;

pub trait Text<C: Cursor>: Debug {
    fn source(&self) -> &str;
    fn new_cursor(&self) -> C;
}