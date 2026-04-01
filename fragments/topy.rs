// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use pyo3::*;
use pyo3::prelude::*;
use std::any::Any;

/// A trait for Rust-side nodes that know how to cross into Python.
pub trait ToPy {
    fn to_py(&self, py: Python<'_>) -> PyResult<PyObject>;
}

// We implement ToPy for common types so the bridge is automatic.
impl ToPy for String {
    fn to_py(&self, py: Python<'_>) -> PyResult<PyObject> {
        Ok(self.into_py(py))
    }
}