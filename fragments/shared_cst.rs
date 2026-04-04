impl Cst {
    /// The public entry point.
    pub fn node(self) -> Rc<Self> {
        let factory = CstConstructor::new();
        let (tags, ovr, tree) = self._distill(&factory);

        // At this point, 'tags' is guaranteed to contain
        // only Rc<Cst> nodes produced by 'factory'.

        // 1. Merge overrides into the main tree
        let final_cst = tree.apply_overrides(ovr);

        // 2. If an Ast was produced, wrap it in a shared node
        if !tags.is_empty() {
             factory.shared(Cst::Ast(Box::new(tags)))
        } else {
             factory.shared(final_cst)
        }
    }

    fn _distill(self, factory: &CstConstructor) -> (Ast, Cst, Cst) {
        match self {
            Cst::List(elements) => {
                let mut tags = Ast::new();
                let mut ovr = Cst::Nil;
                let mut tree = Cst::Nil;

                for node in elements {
                    let (child_ast, child_ovr, child_cst) = node._distill(factory);
                    tags.update(child_ast);
                    ovr = ovr.merge(child_ovr);
                    tree = tree.merge(child_cst);
                }
                (tags, ovr, tree)
            }
            // Because this is the ONLY way to set a value in Ast,
            // we call factory.shared() right here.
            Cst::Named(keyval) => {
                let (name, val) = *keyval;
                let mut tags = Ast::new();
                tags.set(name, factory.shared(val));
                (tags, Cst::Nil, Cst::Nil)
            }
            // ... other variants ...
            Cst::Nil => (Ast::new(), Cst::Nil, Cst::Nil),
            other => (Ast::new(), Cst::Nil, other),
        }
    }
}
