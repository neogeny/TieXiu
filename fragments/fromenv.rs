use std::env;

impl Cfg {
    pub fn fromenv(prefix: &str) -> Self {
        let mut collected: Vec<(Box<str>, Box<str>)> = Vec::new();

        // We look for "PREFIX" or "PREFIX_"
        let prefix_with_underscore = format!("{}_", prefix);

        for (key, value) in env::vars() {
            if key.starts_with(prefix) {
                let mangled_name = if key.starts_with(&prefix_with_underscore) {
                    // Rule: remove the single underscore following the prefix
                    &key[prefix_with_underscore.len()..]
                } else {
                    // Rule: just remove the prefix
                    &key[prefix.len()..]
                };

                // Rule: convert the remaining name to lowercase
                let final_key = mangled_name.to_lowercase().into_boxed_str();
                let final_val = value.into_boxed_str();

                collected.push((final_key, final_val));
            }
        }

        // Freeze the Vec into the ultra-lean Boxed Slice
        Self {
            pairs: collected.into_boxed_slice(),
        }
    }
}use std::env;

impl Cfg {
    pub fn fromenv(prefix: &str) -> Self {
        let pairs = env::vars()
            // 1. Filter: Only variables intended for this prefix
            .filter(|(key, _)| key.starts_with(prefix))

            // 2. Map: Peel the prefix
            .map(|(key, val)| (key[prefix.len()..].to_string(), val))

            // 3. Map: Peel the optional leading underscore (Key-only logic)
            .map(|(mut key, val)| {
                if key.starts_with('_') {
                    key.remove(0);
                }
                (key, val)
            })

            // 4. Map: Normalize to lowercase
            .map(|(key, val)| (key.to_lowercase(), val))

            // 5. Map: Finalize into the lean boxed pairs
            .map(|(k, v)| (k.into_boxed_str(), v.into_boxed_str()))

            // 6. Freeze into the Cfg's optimized storage
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self { pairs }
    }
}
