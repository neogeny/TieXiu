use crate::engine::ctx::{Ctx, ParseResult};
use super::model::Model;

pub struct Sequence {
    pub exps: Vec<Box<dyn Model>>,
}

impl Sequence {
    /// Accepts a Vec of anything that can be turned into a Box<dyn Model>
    pub fn new(exps: Vec<Box<dyn Model>>) -> Self {
        Self { exps }
    }
    
    fn parse(&self, ctx: &mut Ctx) -> ParseResult {
        let mut results = Vec::new();
        for exp in &self.exps {
            // The '?' operator is the key: 
            // it exits early and returns the Err if any element fails.
            results.push(exp.parse(ctx)?);
        }
        Err("Not implemented".to_string())
        // Ok(ParsedValue::new(results))
    }
}