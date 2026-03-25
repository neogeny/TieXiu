use crate::engine::cst::Cst;
use crate::engine::ctx::Ctx;
use super::model::Model;

pub struct Group {
    pub exp: Box<dyn Model>,
}

impl Model for Group {
    fn parse(&self, ctx: Ctx) -> Result<(Ctx, Cst), String> {
        self.exp.parse(ctx)
    }
}
