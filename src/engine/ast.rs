// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use super::cst::Cst;

pub const __AT__: &str = "&";

/// A structured mapping for AST nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ast {
    pub fields: HashMap<String, Cst>,
}

impl Ast {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safekey(k);
            self.fields.entry(key).or_insert(Cst::Void);
        }

        for &k in list_keys {
            let key = self.safekey(k);
            self.fields.entry(key).or_insert(Cst::List(Vec::new()));
        }
    }

    pub fn set(&mut self, key: &str, item: Cst) {
        let key = self.safekey(key);
        let mut new = item;
        if let Some(current) = self.fields.remove(&key) {
            new = current.add(new)
        }
        self.fields.insert(key, new);
    }
    
    pub fn set_list(&mut self, key: &str, item: Cst) {
        let key = self.safekey(key);
        let mut new = item;
        if let Some(current) = self.fields.remove(&key) {
            new = current.addlist(new)
        }
        self.fields.insert(key, new);
    }

    fn safekey(&self, key: &str) -> String {
        let mut k = key.to_string();
        while self.is_unsafe(&k) {
            k.push('_');
        }
        k
    }

    fn is_unsafe(&self, key: &str) -> bool {
        matches!(key, "items" | "keys" | "values" | "get" | "update" | "pop" | "clear")
    }
}