use std::any::Any;

pub type Node = Box<dyn Any>;

pub enum Cst {
    List(Vec<Node>), // The "Open" list
    Item(Node),      // A single item or a "Closed" list
}

/// Equivalent to cstfinal()
pub fn cst_final(cst: Cst) -> Cst {
    match cst {
        // To "Close" a list, we wrap the Vec into a single Node
        Cst::List(v) => Cst::Item(Box::new(v)),
        item => item,
    }
}

pub fn cst_add(cst: Option<Cst>, node: Node, as_list: bool) -> Cst {
    match (cst, as_list) {
        (None, true) => Cst::List(vec![node]),
        (None, false) => Cst::Item(node),
        (Some(Cst::List(mut v)), _) => {
            v.push(node);
            Cst::List(v)
        }
        (Some(Cst::Item(i)), _) => Cst::List(vec![i, node]),
    }
}

pub fn cst_merge(cst: Option<Cst>, other: Option<Cst>) -> Option<Cst> {
    match (cst, other) {
        (None, None) => None,
        (None, Some(o)) => Some(o),
        (Some(c), None) => Some(c),
        // Both are open lists: extend
        (Some(Cst::List(mut c_vec)), Some(Cst::List(o_vec))) => {
            c_vec.extend(o_vec);
            Some(Cst::List(c_vec))
        }
        // Left is open, right is an item
        (Some(Cst::List(mut c_vec)), Some(Cst::Item(o))) => {
            c_vec.push(o);
            Some(Cst::List(c_vec))
        }
        // Left is item, right is open
        (Some(Cst::Item(c)), Some(Cst::List(mut o_vec))) => {
            let mut new_vec = Vec::with_capacity(o_vec.len() + 1);
            new_vec.push(c);
            new_vec.append(&mut o_vec);
            Some(Cst::List(new_vec))
        }
        // Both are items
        (Some(Cst::Item(c)), Some(Cst::Item(o))) => Some(Cst::List(vec![c, o])),
    }
}