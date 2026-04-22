// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Memento(u64);

impl Default for Memento {
    fn default() -> Self {
        Self::new()
    }
}

impl Memento {
    pub fn new() -> Self {
        // A global, static counter that lives for the duration of the program
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        // Fetch the current value and increment it by 1 atomically
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        Memento(id)
    }

    pub fn id(&self) -> u64 {
        self.0
    }
}
