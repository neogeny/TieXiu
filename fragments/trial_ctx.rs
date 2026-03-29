impl<'c, C: Ctx> Ctx<'c> {
    pub fn group<F, T>(&mut self, body: F) -> Result<T, String>
    where
        F: FnOnce(&mut Self) -> Result<T, String>,
    {
        // 1. Create a "Trial" version of the context (copy/clone)
        let mut trial = self.clone();

        // 2. Execute the grammar block on the trial context
        match body(&mut trial) {
            Ok(parsed) => {
                // 3. SUCCESS: "Merge" by simply overwriting our state
                *self = trial;
                Ok(parsed)
            }
            Err(err) => {
                // 4. FAILURE: Do nothing. 'trial' is dropped.
                // Our 'self' is still at the pre-group position.
                Err(err)
            }
        }
    }
}
