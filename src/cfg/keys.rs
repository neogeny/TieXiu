// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

// NOTE! Order matters here! Debug < Mode < Trace
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum ConfigKey {
    Debug,
    Verbose,
    Trace,
}

#[cfg(test)]
mod test {
    use super::ConfigKey;
    use std::collections::BTreeSet;

    #[test]
    fn bree_test() {
        // 1. Using a BTreeSet
        let mut set = BTreeSet::new();
        set.insert(ConfigKey::Trace);
        set.insert(ConfigKey::Debug);

        // The set automatically sorts them: [Debug, Trace]
        for key in &set {
            println!("{:?}", key);
        }

        // 2. Using Binary Search on a Vec
        let mut keys = [ConfigKey::Trace, ConfigKey::Debug, ConfigKey::Verbose];

        // Binary search ONLY works on sorted slices
        keys.sort();

        let search_result = keys.binary_search(&ConfigKey::Verbose);

        match search_result {
            Ok(index) => println!("Found Mode at index {}", index),
            Err(_) => println!("Mode not found"),
        }
    }
}
