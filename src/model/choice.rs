use crate::engine::{Cst, Ctx};
use super::model::CanParse;


pub struct Choice<M: CanParse> {
    pub options: Vec<Box<M>>,
}

impl<M: CanParse> CanParse for Choice<M> {
    fn parse(&self, ctx: Ctx) -> Result<(Ctx, Cst), (Ctx, String)> {
        let mut furthest_err: (Ctx, String) = (
            Ctx{offset: 0, cut_seen: false}, 
           String::new()
        );

        for option in &self.options {
            match option.parse(ctx) {
                Ok(res) => return Ok(res),
                Err((mut err_ctx, msg)) => {
                    err_ctx.cut_seen = false;
                    if err_ctx.cut_seen {
                        return Err((err_ctx, msg));
                    }

                    if err_ctx.offset >= furthest_err.0.offset {
                        furthest_err = (err_ctx, msg);
                    }
                }
            }
        }
        Err(furthest_err)
    }
}
