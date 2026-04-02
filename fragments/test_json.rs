#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_grammar_round_trip() {
        let data = include_str!("../grammar/tatsu.json");

        // Step 1: Input -> ModelNode
        let model: ModelNode = serde_json::from_str(data).expect("Failed to parse JSON");

        // Step 2: ModelNode -> Output (String)
        let output = serde_json::to_string_pretty(&model).expect("Failed to serialize");

        // Step 3: Verify Substance
        // Note: You might need to re-parse 'output' back to a Value to compare objects
        // directly if key order in the original JSON isn't sorted.
        let v1: serde_json::Value = serde_json::from_str(data).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(v1, v2);
    }
}
