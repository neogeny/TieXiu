use std::rc::Rc;

pub struct CstInterner {
    void: Rc<Cst>,
    nil: Rc<Cst>,
    bottom: Rc<Cst>,
}

impl CstInterner {
    pub fn new() -> Self {
        Self {
            void: Rc::new(Cst::Void),
            nil: Rc::new(Cst::Nil),
            bottom: Rc::new(Cst::Bottom),
        }
    }

    /// The true "interning" gatekeeper.
    /// Since _distill handles recursion, this only handles the promotion
    /// of a single node level to a shared Rc.
    pub fn intern(&self, node: Cst) -> Rc<Cst> {
        match node {
            Cst::Void => Rc::clone(&self.void),
            Cst::Nil => Rc::clone(&self.nil),
            Cst::Bottom => Rc::clone(&self.bottom),

            // Data-carrying nodes (including List/Ast/Named) are already
            // prepared by _distill. We just put them on the heap once.
            _ => Rc::new(node),
        }
    }

    // Direct accessors for the "empties" to be used during _distill's logic
    pub fn void(&self) -> Rc<Cst> {
        Rc::clone(&self.void)
    }

    pub fn nil(&self) -> Rc<Cst> {
        Rc::clone(&self.nil)
    }

    pub fn bottom(&self) -> Rc<Cst> {
        Rc::clone(&self.bottom)
    }
}
