use crate::input::Cursor;
use crate::engine::{Cst, Ctx};
use super::model::{CanParse, ParseResult};

pub struct Sequence<M> {
    pub exps: Vec<Box<M>>,
}

impl<M, C> CanParse<C> for Sequence<M>
where
    M: CanParse<C>,
    C: Cursor,
{
    fn parse(&self, mut ctx: Ctx<C>) -> ParseResult<C> {
        let mut results = Vec::new();
        for exp in &self.exps {
            let (new_ctx, cst) = exp.parse(ctx)?;
            ctx = new_ctx;
            results.push(cst)
        }
        Ok(
            (ctx, Cst::from(results))
        )
    }
}