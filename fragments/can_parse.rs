pub struct Sequence<M, C>
where
    M: CanParse<C>,
    C: Cursor,
{
    pub children: Vec<Box<M>>,
    _phantom: std::marker::PhantomData<C>,
}
