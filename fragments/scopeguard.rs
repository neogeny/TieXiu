// Copyright (c) 2026 Juancarlo Añez
// SPDX-License-Identifier: MIT OR Apache-2.0

struct ScopeGuard<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for ScopeGuard<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

pub fn example_operation() {
    // Logic starts here
    let _guard = ScopeGuard(Some(|| {
        // This is your "finally" block
        println!("Cleanup complete.");
    }));

    // If any code here returns early or panics, _guard is dropped.
}
