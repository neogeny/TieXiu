impl Cst {
    /// The public entry point.
    pub fn node(self) -> Rc<Self> {
        let factory = CstConstructor::new();
        let (ast, ovr, cst) = self._distill(&factory);

        // At this point, 'ast' is guaranteed to contain
        // only Rc<Cst> nodes produced by 'factory'.

        // 1. Merge overrides into the main cst
        let final_cst = cst.apply_overrides(ovr);

        // 2. If an Ast was produced, wrap it in a shared node
        if !ast.is_empty() {
             factory.shared(Cst::Ast(Box::new(ast)))
        } else {
             factory.shared(final_cst)
        }
    }

    fn _distill(self, factory: &CstConstructor) -> (Ast, Cst, Cst) {
        match self {
            Cst::List(elements) => {
                let mut ast = Ast::new();
                let mut ovr = Cst::Nil;
                let mut cst = Cst::Nil;

                for node in elements {
                    let (child_ast, child_ovr, child_cst) = node._distill(factory);
                    ast.update(child_ast);
                    ovr = ovr.merge(child_ovr);
                    cst = cst.merge(child_cst);
                }
                (ast, ovr, cst)
            }
            // Because this is the ONLY way to set a value in Ast,
            // we call factory.shared() right here.
            Cst::Named(keyval) => {
                let (name, val) = *keyval;
                let mut ast = Ast::new();
                ast.set(name, factory.shared(val));
                (ast, Cst::Nil, Cst::Nil)
            }
            // ... other variants ...
            Cst::Nil => (Ast::new(), Cst::Nil, Cst::Nil),
            other => (Ast::new(), Cst::Nil, other),
        }
    }
}
