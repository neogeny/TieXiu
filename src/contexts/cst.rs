// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ast::Ast;
use super::json::Json;
pub use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
pub enum Cst {
    Token(String),
    Literal(String),
    Number(f64),
    List(Vec<Cst>),
    Closure(Vec<Cst>),
    Named(String, Box<Cst>),
    NamedList(String, Box<Cst>),
    OverrideValue(Box<Cst>),
    OverrideList(Box<Cst>),
    Ast(Ast),
    Nil,
    Bottom,
}

pub fn cst_add(prev: Cst, node: Cst) -> Cst {
    prev.add(node)
}

pub fn cst_addlist(prev: Cst, node: Cst) -> Cst {
    prev.addlist(node)
}

pub fn cst_merge(prev: Cst, node: Cst) -> Cst {
    prev.merge(node)
}

pub fn cst_closed(cst: Cst) -> Cst {
    cst.closed()
}


impl From<Vec<Cst>> for Cst {
    fn from(v: Vec<Cst>) -> Self {
        Cst::List(v)
    }
}

impl<const N: usize> From<[Cst; N]> for Cst {
    fn from(arr: [Cst; N]) -> Self {
        Cst::List(arr.into())
    }
}

impl From<&Cst> for Json {
    fn from(cst: &Cst) -> Self {
        cst.to_json()
    }
}


impl Add for Cst {
    type Output = Self;

    fn add(self, node: Self) -> Self::Output {
        match self {
            Cst::Nil => node,
            Cst::List(mut list) => {
                list.push(node);
                Cst::List(list)
            }
            _ => Cst::List(vec![self, node]),
        }
    }
}

impl Cst {
    pub fn addlist(self, node: Cst) -> Cst {
        match self {
            Cst::Nil => Cst::List(vec![node]),
            Cst::List(mut list) => {
                list.push(node);
                Cst::List(list)
            }
            _ => Cst::List(vec![self, node]),
        }
    }

    pub fn merge(self, node: Cst) -> Cst {
        match (self, node) {
            (Cst::List(mut list), Cst::List(other_list)) => {
                list.extend(other_list);
                Cst::List(list)
            }
            (Cst::List(mut list), other_node) => {
                list.push(other_node);
                Cst::List(list)
            }
            (self_node, Cst::List(mut other_list)) => {
                other_list.insert(0, self_node);
                Cst::List(other_list)
            }
            (s, n) => s.add(n),
        }
    }

    pub fn closed(self) -> Cst {
        match self {
            Cst::List(list) => Cst::Closure(list),
            _ => self,
        }
    }

    fn _distill(self) -> (Ast, Cst, Cst) {
        let mut ast = Ast::new();
        let mut ovr = Cst::Nil;
        let mut cst = Cst::Nil;

        match self {
            Cst::List(elements) => {
                for node in elements {
                    let (child_ast, child_ovr, child_cst) = node._distill();

                    ast.update(&child_ast);
                    ovr = ovr.merge(child_ovr);
                    cst = cst.merge(child_cst);
                }
            }

            Cst::Named(name, val) => ast.set(&name, *val),
            Cst::NamedList(name, val) => ast.set_list(&name, *val),
            Cst::OverrideValue(val) => ovr = ovr.add(*val),
            Cst::OverrideList(val) => ovr = ovr.addlist(*val),

            Cst::Nil => {}

            other => cst = cst.merge(other),
        }

        (ast, ovr, cst)
    }

    pub fn node(self) -> Cst {
        let (ast, ovr, cst) = self._distill();

        // Priority Gate: Override > AST > CST
        if ovr != Cst::Nil {
            cst_closed(ovr)
        } else if !ast.is_empty() {
            Cst::Ast(ast)
        } else {
            cst_closed(cst)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    const TARGET: usize = 64;

    #[test]
    fn test_cst_size() {
        let size = size_of::<Cst>();
        // 24 bytes: Box (8) + Rc (8) + bool/padding (8)
        assert!(size <= TARGET, "Cst size is {} > {} bytes", size, TARGET);
    }

    // Setup the 4 base variants
    fn nil() -> Cst { Cst::Nil }
    fn some() -> Cst { Cst::Number(1.0) }
    fn list() -> Cst { Cst::List(vec![Cst::Number(2.0)]) }
    fn closed() -> Cst { Cst::Closure(vec![Cst::Number(3.0)]) }

    #[test]
    fn test_cst_add() {
        // Examples of the matrix logic:
        
        // 1. Nil + Nil -> Nil
        assert_eq!(cst_add(nil(), nil()), nil());
        
        // 2. Nil + Something -> Something (Identity)
        assert_eq!(cst_add(nil(), some()), some());
        
        // 6. Something + Something -> List (Promotion)
        if let Cst::List(v) = cst_add(some(), some()) {
            assert_eq!(v.len(), 2);
        } else { panic!("Should promote to List"); }

        // 11. List + List -> List (Concatenation)
        if let Cst::List(v) = cst_add(list(), list()) {
            assert_eq!(v.len(), 2);
        }

        // 16. Closed + Closed -> List([Closed, Closed])
        // Usually, Closed is 'atomic' to the next layer up
        let res = cst_add(closed(), closed());
        assert!(matches!(res, Cst::List(_)));
    }

    #[test]
    fn test_cst_addlist() {
        // addlist forces a List even on the first item
        
        // 1. Nil + Nil -> List([]) or Nil depending on your preference
        // 2. Nil + Something -> List([Something])
        if let Cst::List(v) = cst_addlist(nil(), some()) {
            assert_eq!(v.len(), 1);
        }

        // 6. Something + Something -> List([S, S])
        let res = cst_addlist(some(), some());
        assert!(matches!(res, Cst::List(_)));
    }

    #[test]
    fn test_cst_merge() {
        // Merge is the logic for building Objects/Asts.
        // If we merge two 'Somethings', do they become a List or a new Ast?
        let res = cst_merge(some(), some());
        // Simple case: merges into a collection
        assert!(matches!(res, Cst::List(_)));
    }

    #[test]
    fn test_cst_closed() {
        // This is a unary-like check across the states. 
        // We ensure that 'closing' any of the 4 types produces the correct 'Closed' state.
        
        assert_eq!(cst_closed(nil()), nil());
        assert!(matches!(cst_closed(some()), Cst::Number(_))); // Usually Something stays as is
        assert!(matches!(cst_closed(list()), Cst::Closure(_))); // List becomes Closure
        assert!(matches!(cst_closed(closed()), Cst::Closure(_))); // Idempotent
    }
}