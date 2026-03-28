use crate::engine::{Cst, Ctx};
use super::model::CanParse;

pub struct Sequence<M: CanParse> {
    pub exps: Vec<Box<M>>,
}

impl<M: CanParse> Sequence<M> {
    pub fn new(exps: Vec<Box<M>>) -> Self {
        Self { exps }
    }
    
}

impl<M: CanParse> CanParse for Sequence<M> {
    fn parse(&self, mut ctx: Ctx) -> Result<(Ctx, Cst), (Ctx, String)> {
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