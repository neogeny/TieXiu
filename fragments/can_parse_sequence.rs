// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

// 1. The data: Just a collection of models.
// No mention of a Cursor here.
pub struct Sequence<M> {
    pub children: Vec<Box<M>>,
}

// 2. The capability: The implementation is where
// the relationship with the Cursor is defined.
impl<M, C> CanParse<C> for Sequence<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        let mut elements = Vec::with_capacity(self.children.len());

        for child in &self.children {
            // The recursion happens here.
            // C is inferred from the ctx passed in.
            let (next_ctx, child_cst) = child.parse(ctx)?;
            ctx = next_ctx;

            match child_cst {
                Cst::Nil => continue,
                other => elements.push(Box::new(other)),
            }
        }

        Ok((ctx, Cst::List(elements)))
    }
}
