impl<'a> Cursor<'a> {
    pub fn parse_choice(&mut self, rules: &[fn(&Cursor<'a>) -> Result<Cst<'a>, String>]) -> Result<Cst<'a>, String> {
        for rule in rules {
            // 1. Clone the current state
            let mut trial_cursor = self.clone();

            // 2. Try the rule on the clone
            if let Ok(result) = rule(&mut trial_cursor) {
                // 3. SUCCESS: Reassign the main cursor to the advanced state
                *self = trial_cursor;
                return Ok(result);
            }
            // 4. FAILURE: trial_cursor is dropped, 'self' remains unchanged
        }
        Err("No rules matched".into())
    }
}
