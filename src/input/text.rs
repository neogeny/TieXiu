// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Cursor;
use std::fmt::Debug;

pub trait Text<C: Cursor>: Debug {
    fn source(&self) -> &str;
    fn new_cursor(&self) -> C;
}
