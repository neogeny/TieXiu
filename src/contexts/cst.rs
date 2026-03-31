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
    Void,
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
        match (self, node) {
            // Identity: Nil + node = node
            (Cst::Nil, node) => node,
            // Identity: self + Nil = self
            (s, Cst::Nil) => s,

            (Cst::List(mut list), node) => {
                list.push(node);
                Cst::List(list)
            }
            (s, n) => Cst::List(vec![s, n]),
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

    const TARGET: usize = 16;

    #[test]
    fn test_cst_size() {
        let size = size_of::<Cst>();
        // 24 bytes: Box (8) + Rc (8) + bool/padding (8)
        assert!(size <= TARGET, "Cst size is {} > {} bytes", size, TARGET);
    }


    #[test]
    fn test_node_nil_removal() {
        // Input: List([Nil, Bottom, Nil])
        // Step 1: cst = merge(Nil, Nil) -> Nil
        // Step 2: cst = merge(Nil, Bottom) -> Bottom
        // Step 3: cst = merge(Bottom, Nil) -> Bottom (The Identity Law)
        let raw = Cst::List(vec![Cst::Nil, Cst::Bottom, Cst::Nil]);
        let result = raw.node();

        // result = cst_closed(Bottom)
        assert_eq!(result, cst_closed(Cst::Bottom));
    }

    #[test]
    fn test_node_nil_removal_to_bottom() {
        let raw = Cst::List(vec![Cst::Nil, Cst::Bottom, Cst::Nil]);
        let result = raw.node();

        // If cst_closed is identity for non-lists, this is just Bottom
        assert_eq!(result, Cst::Bottom);
    }

    #[test]
    fn test_node_nil_removal_to_list() {
        // Input: [Bottom, Nil, Bottom]
        // Step 1: cst = merge(Bottom, Nil) -> Bottom
        // Step 2: cst = merge(Bottom, Bottom) -> List([Bottom, Bottom])
        let raw = Cst::List(vec![Cst::Bottom, Cst::Nil, Cst::Bottom]);
        let result = raw.node();

        if let Cst::Closure(v) = result {
            assert_eq!(v.len(), 2); // Nil is gone, only the two Bottoms remain
            assert_eq!(v[0], Cst::Bottom);
            assert_eq!(v[1], Cst::Bottom);
        } else {
            panic!("Expected Closure, got {:?}", result);
        }
    }

    #[test]
    fn test_node_nil_purging_preserves_count() {
        // Input: List([Nil, Bottom, Nil])
        let raw = Cst::List(vec![Cst::Nil, Cst::Bottom, Cst::Nil]);
        let result = raw.node();

        // Since it's effectively Bottom, and Bottom isn't a list,
        // it doesn't become a Closure of len 1. It just stays Bottom.
        assert_eq!(result, Cst::Bottom);
    }
}