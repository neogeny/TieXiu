// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::error::{Error, Result};
use std::collections::BTreeMap;
use std::ops::Index;
use std::str::FromStr;
use std::{env, fmt};

/// Python-style falsy values for string-based configuration.
pub const FALSY_VALUES: &[&str] = &["false", "0", "no", "none", "False", "No"];

/// Helper to determine if a string is "falsy" in a Pythonic context.
pub fn is_falsy(v: &str) -> bool {
    v.is_empty() || FALSY_VALUES.contains(&v.to_lowercase().as_str())
}

/// What gets passed around
pub type CfgA<'c> = &'c [(&'c str, &'c str)];
pub type CfgR<'c> = Box<[(&'c str, &'c str)]>;

/// An owned, optimized configuration container.
/// Invariants:
/// 1. Sorted by key (for binary search).
/// 2. Unique keys (no duplicates).
/// 3. Latter wins (last key in input overrides previous ones).
#[derive(Clone, Default)]
pub struct Cfg {
    pairs: Box<[(Box<str>, Box<str>)]>,
}

/// You can also implement IntoIterator for references to allow:
/// for (k, v) in &cfg { ... }
impl<'a> IntoIterator for &'a Cfg {
    type Item = (&'a str, &'a str);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

/// Has use for a Cfg
pub trait Configurable {
    fn configure(&mut self, cfg: &Cfg);
}

impl Cfg {
    /// Creates a new Cfg ensuring all invariants (Sorted, Unique, Latter Wins).
    pub fn new(pairs: CfgA) -> Self {
        // BTreeMap enforces uniqueness and sorting.
        // Collecting an iterator into a BTreeMap ensures the last value wins.
        let map: BTreeMap<Box<str>, Box<str>> = pairs
            .iter()
            .map(|(k, v)| (Box::from(*k), Box::from(*v)))
            .collect();

        Self {
            pairs: map.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn check_new(pairs: CfgA, valid: &[&str]) -> Result<Self> {
        for (name, _) in pairs {
            if !valid.contains(name) {
                return Err(Error::UnknownCfgOption((**name).into()));
            }
        }
        Ok(Self::new(pairs))
    }

    pub fn from_env(prefix: &str) -> Self {
        let map: BTreeMap<Box<str>, Box<str>> = env::vars()
            .filter(|(key, _)| key.starts_with(prefix))
            .map(|(key, val)| {
                let mut key = key[prefix.len()..].to_string();
                if key.starts_with('_') {
                    key.remove(0);
                }
                (key.to_lowercase().into_boxed_str(), val.into_boxed_str())
            })
            .collect();

        Self {
            pairs: map.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn from_boxed_slice(pairs: Box<[(&'_ str, &'_ str)]>) -> Self {
        Self::new(pairs.as_ref())
    }

    pub fn as_boxed_slice(&self) -> Box<[(&str, &str)]> {
        self.pairs
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// Merges two configurations. Values from 'other' win on key collisions.
    pub fn merge(&self, other: &Cfg) -> Self {
        let mut map: BTreeMap<Box<str>, Box<str>> = self.pairs.iter().cloned().collect();

        for (k, v) in other.pairs.iter() {
            // Keep existing if it's truthy, otherwise override.
            if let Some(u) = map.get(k)
                && !is_falsy(u)
            {
                continue;
            }
            map.insert(k.clone(), v.clone());
        }

        Self {
            pairs: map.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        // 1. Convert existing pairs to a map
        let mut map: BTreeMap<Box<str>, Box<str>> = self.pairs
            .iter()
            .cloned()
            .collect();

        // 2. Insert/Overwrite the new value
        map.insert(Box::from(key), Box::from(value));

        // 3. Re-freeze into a boxed slice
        self.pairs = map.into_iter().collect::<Vec<_>>().into_boxed_slice();
    }

    /// Returns an iterator over the configuration pairs as borrowed strings.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.pairs
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.pairs
            .binary_search_by(|(k, _)| k.as_ref().cmp(key))
            .ok()
            .map(|idx| self.pairs[idx].1.as_ref())
    }

    pub fn is_enabled(&self, key: &str) -> bool {
        self.get(key).map(|v| !is_falsy(v)).unwrap_or(false)
    }

    pub fn get_or<T: FromStr>(&self, key: &str, default: T) -> T {
        self.get(key)
            .and_then(|v| v.parse::<T>().ok())
            .unwrap_or(default)
    }

    pub fn get_value(&self, key: &str) -> &str {
        self.get(key).unwrap_or("")
    }
}

impl Index<&str> for Cfg {
    type Output = str;
    fn index(&self, key: &str) -> &Self::Output {
        self.get_value(key)
    }
}

impl fmt::Debug for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_map = f.debug_map();
        for (k, v) in self.pairs.iter() {
            debug_map.entry(&k.as_ref(), &v.as_ref());
        }
        debug_map.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latter_wins_invariant() {
        // "trace" is defined twice. The second one ("1") must win.
        let storage = [("trace", "0"), ("mode", "strict"), ("trace", "1")];
        let cfg = Cfg::new(&storage);

        assert_eq!(cfg.pairs.len(), 2); // Unique keys
        assert_eq!(cfg.get_value("trace"), "1"); // Latter wins
        // Verify sorting (a < m < t is not the case here, but 'm' < 't')
        assert_eq!(cfg.pairs[0].0.as_ref(), "mode");
        assert_eq!(cfg.pairs[1].0.as_ref(), "trace");
    }

    #[test]
    fn test_binary_search_get() {
        let cfg = Cfg::new(&[("z", "last"), ("a", "first"), ("m", "middle")]);
        assert_eq!(cfg.get("a"), Some("first"));
        assert_eq!(cfg.get("m"), Some("middle"));
        assert_eq!(cfg.get("z"), Some("last"));
        assert_eq!(cfg.get("missing"), None);
    }

    #[test]
    fn test_merge_logic() {
        let base = Cfg::new(&[("a", "0"), ("b", "1")]);
        let over = Cfg::new(&[("a", "1"), ("c", "2")]);
        let merged = base.merge(&over);

        // "a" was truthy "0" in Pythonic terms? No, "0" is falsy.
        // So "a" should be overridden by "1".
        assert_eq!(&merged["a"], "1");
        assert_eq!(&merged["b"], "1");
        assert_eq!(&merged["c"], "2");
    }
}
