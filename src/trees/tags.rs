// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tree::Tree;
use std::collections::HashMap;
use std::ops::Add;

/// A structured mapping for AST nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeTags {
    pub tags: HashMap<String, Tree>,
}

impl TreeTags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&Tree> {
        self.tags.get(key)
    }

    pub fn update(&mut self, other: &TreeTags) {
        for (key, value) in &other.tags {
            self.tags.insert(key.clone(), value.clone());
        }
    }

    pub fn define(&mut self, keys: &[&str], list_keys: &[&str]) {
        for &k in keys {
            let key = self.safekey(k);
            self.tags.entry(key).or_insert(Tree::Nil);
        }

        for &k in list_keys {
            let key = self.safekey(k);
            self.tags.entry(key).or_insert(Tree::Node([].into()));
        }
    }

    pub fn set(&mut self, key: &str, item: Tree) {
        let key = self.safekey(key);
        if let Some(current) = self.tags.remove(&key) {
            let new = current.add(item);
            self.tags.insert(key, new);
        } else {
            self.tags.insert(key, item);
        }
    }

    pub fn set_list(&mut self, key: &str, item: Tree) {
        let key = self.safekey(key);
        if let Some(current) = self.tags.remove(&key) {
            let new = current.add_node(item);
            self.tags.insert(key, new);
        } else {
            self.tags.insert(key, item);
        }
    }

    fn safekey(&self, key: &str) -> String {
        let mut k = key.to_string();
        while self.is_unsafe(&k) {
            k.push('_');
        }
        k
    }

    fn is_unsafe(&self, key: &str) -> bool {
        matches!(
            key,
            "items" | "keys" | "values" | "get" | "update" | "pop" | "clear"
        )
    }
}
