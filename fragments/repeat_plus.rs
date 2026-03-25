impl Ctx {
    /// The "Zero or More" (* closure)
    pub fn repeat<F>(&self, parse_fn: F) -> (Ctx, Cst)
    where
        F: Fn(Ctx) -> Result<(Ctx, Cst), (bool, usize)>,
    {
        let mut current_ctx = *self;
        let mut results = Vec::new();

        while let Ok((next_ctx, cst)) = parse_fn(current_ctx) {
            if next_ctx.offset == current_ctx.offset { break; } // Avoid infinite loops
            current_ctx = next_ctx;
            results.push(cst);
        }
        (current_ctx, Cst::Closed(results))
    }

    /// The "One or More" (+ closure)
    pub fn repeat_plus<F>(&self, parse_fn: F) -> Result<(Ctx, Cst), (bool, usize)>
    where
        F: Fn(Ctx) -> Result<(Ctx, Cst), (bool, usize)>,
    {
        // Require the first match
        let (mut current_ctx, first_cst) = parse_fn(*self)?;
        let mut results = vec![first_cst];

        // Then delegate to the standard repeat logic for the rest
        let (final_ctx, Cst::Closed(mut rest)) = current_ctx.repeat(parse_fn) else {
            unreachable!()
        };
        results.append(&mut rest);

        Ok((final_ctx, Cst::Closed(results)))
    }
}
