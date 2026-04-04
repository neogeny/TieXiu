impl<'a> From<Cst<'a>> for Box<Cst<'a>> {
    #[inline(always)]
    fn from(tree: Cst<'a>) -> Self {
        Box::new(tree)
    }
}
