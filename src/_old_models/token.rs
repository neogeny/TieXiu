use crate::grammars::{CanParse, ParseResult};
use crate::contexts::{Cst, Ctx};


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

impl<C: Ctx> CanParse<C> for Token
{
    fn parse(&self, ctx: C) -> ParseResult<C> {
        if ctx.token(&self.token) {
            Ok((ctx, Cst::Token(self.token.clone())))
        }
        else { 
            Err(ctx)
        }
    }
}


