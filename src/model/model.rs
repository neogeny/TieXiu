use crate::engine::{Cst, Ctx};

pub trait Model {
    fn parse(&self, mut ctx: Ctx) -> Result<(Ctx, Cst), (bool, usize, String)> {
        unimplemented!()
    }
}