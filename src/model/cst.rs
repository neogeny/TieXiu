use std::any::Any;

pub type Node = Box<dyn Any>;

pub enum Cst {
    List(Vec<Node>),
    Item(Node),
}

pub fn cst_final(cst: Cst) -> Cst {
    match cst {
        Cst::List(v) => Cst::Item(Box::new(v)),
        item => item,
    }
}

pub fn cst_add<T: Any>(cst: Option<Cst>, item: T, as_list: bool) -> Cst {
    let node: Node = Box::new(item);

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

pub fn cst_merge<T: Any>(cst: Option<Cst>, other: T) -> Cst {
    // We start by boxing the 'other' item.
    let mut other_any: Node = Box::new(other);

    // If 'other' is already a Cst, we extract it to handle merging/flattening.
    // Otherwise, we treat it as a raw Node (Item).
    let other_cst = if let Some(extracted) = other_any.downcast::<Cst>().ok() {
        *extracted
    } else {
        Cst::Item(other_any)
    };

    match (cst, other_cst) {
        (None, right) => right,

        (Some(Cst::List(mut l_vec)), Cst::List(r_vec)) => {
            l_vec.extend(r_vec);
            Cst::List(l_vec)
        }
        (Some(Cst::List(mut l_vec)), Cst::Item(r_node)) => {
            l_vec.push(r_node);
            Cst::List(l_vec)
        }
        (Some(Cst::Item(l_node)), Cst::List(mut r_vec)) => {
            let mut new_vec = Vec::with_capacity(r_vec.len() + 1);
            new_vec.push(l_node);
            new_vec.append(&mut r_vec);
            Cst::List(new_vec)
        }
        (Some(Cst::Item(l_node)), Cst::Item(r_node)) => {
            Cst::List(vec![l_node, r_node])
        }
    }
}
