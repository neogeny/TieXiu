impl Ctx {
    /// Executes a parsing function repeatedly until it fails.
    /// Returns the final Ctx and a vector of collected CSTs.
    pub fn repeat<F>(&self, parse_fn: F) -> (Ctx, Vec<Cst>)
    where
        F: Fn(Ctx) -> Result<(Ctx, Cst), (bool, usize)>,
    {
        let mut current_ctx = *self; // Assuming Ctx is Copy or Clone
        let mut results = Vec::new();

        while let Ok((next_ctx, tree)) = parse_fn(current_ctx) {
            // Safety: Ensure we actually progressed to avoid infinite loops
            if next_ctx.offset == current_ctx.offset {
                break;
            }
            current_ctx = next_ctx;
            results.push(tree);
        }

        (current_ctx, results)
    }
}
