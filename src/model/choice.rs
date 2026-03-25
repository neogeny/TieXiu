use crate::engine::cst::Cst;
use crate::engine::ctx::Ctx;
use super::model::Model;

pub struct ChoiceOption {
    exp: Box<dyn Model>
}

pub(crate) struct Choice {
    pub options: Vec<ChoiceOption>,
}

impl Model for Choice {
    fn parse(&self, ctx: Ctx) -> Result<(Ctx, Cst), String> {
        for option in &self.options {
            if let Ok((ctx, cst)) = option.parse(ctx) {
                return Ok((ctx, cst));
            }
        }
        Err("No option matched".to_string())
    }
}

impl ChoiceOption {
    pub fn new(exp: Box<dyn Model>) -> Self {
        Self { exp }
    }
}

impl Model for ChoiceOption {
    fn parse(&self, ctx: Ctx) -> Result<(Ctx, Cst), String> {
        self.exp.parse(ctx)
    }
}