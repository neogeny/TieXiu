// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

impl Parser {
    pub fn enter(&mut self) -> impl FnOnce(impl FnOnce()) + '_ {
        // Setup code
        self.push_state();

        move |block| {
            // Execute the block
            block();

            // Cleanup code
            self.pop_state();
        }
    }
}
