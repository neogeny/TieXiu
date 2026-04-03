// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Cursor;
use std::fmt::Debug;

pub trait Text<U: Cursor>: Debug {
    fn source(&self) -> &str;
    fn new_cursor(&self) -> U;
}
