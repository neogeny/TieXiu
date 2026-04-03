// Inside Ctx / Scope
pub fn error<T>(&self, model: &'c dyn Parser) -> Result<T, CoreCtx<'c, C>> {
    let mut heavy = self.heavy.borrow_mut(); // If using RefCell
    if self.state.cursor.pos() >= heavy.error_pos {
        heavy.error_pos = self.state.cursor.pos();
        heavy.error_model = Some(model);
    }
    Err(self.clone())
}
