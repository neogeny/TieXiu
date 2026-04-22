// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone)]
pub struct Fuse(pub Option<()>);

impl Default for Fuse {
    fn default() -> Self {
        Self(Some(()))
    }
}

impl Fuse {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_good(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_burnt(&self) -> bool {
        self.0.is_none()
    }

    pub fn burn(&mut self) {
        // WARNING: Could panic! here if alread fused
        self.0 = None;
    }
}
