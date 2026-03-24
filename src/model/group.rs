use crate::engine::ctx::{Ctx, ParseResult};
use super::model::Model;

pub struct Group {
    pub exp: Box<dyn Model>,
}

impl Model for Group {
    fn parse(&self, ctx: &mut Ctx) -> ParseResult {
        ctx.group(|c| self.exp.parse(c))
    }
}
