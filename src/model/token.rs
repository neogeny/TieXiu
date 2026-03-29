use crate::model::{CanParse, ParseResult};
use crate::engine::Ctx;


#[derive(Debug, Clone)]
pub struct Token {
    pub token: String
}

impl Token {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}

impl CanParse for Token
{
    fn parse<'a>(&self, ctx: Ctx<'a>) -> ParseResult<'a> {
        ctx.token(&self.token)
    }
}


