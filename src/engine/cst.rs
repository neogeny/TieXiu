// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::ast::Ast;

#[derive(Debug, Clone, PartialEq)]
pub enum Cst<'c> {
    Token(&'c str),
    Literal(&'c str),
    Item(Box<Cst<'c>>),
    List(Vec<Box<Cst<'c>>>),
    Closure(Vec<Box<Cst<'c>>>),
    Named(&'c str, Box<Cst<'c>>),
    NamedList(&'c str, Box<Cst<'c>>),
    OverrideValue(Box<Cst<'c>>),
    OverrideList(Box<Cst<'c>>),
    Ast(Box<Ast<'c>>),
    Nil,
}

impl<'c> Default for Cst<'c> {
    fn default() -> Self {
        Cst::Nil
    }
}

impl<'c> From<Vec<Cst<'c>>> for Cst<'c> {
    fn from(v: Vec<Cst<'c>>) -> Self {
        let boxed = v.into_iter().map(Box::new).collect();
        Cst::List(boxed)
    }
}


impl<'c, const N: usize> From<[Cst<'c>; N]> for Cst<'c> {
    fn from(arr: [Cst<'c>; N]) -> Self {
        let boxed = arr.into_iter().map(Box::new).collect();
        Cst::List(boxed)
    }
}

impl<'c> Cst<'c> {
    pub fn add(self, node: Cst<'c>) -> Cst<'c> {
        let node_box = Box::new(node);
        match self {
            Cst::Nil => *node_box,
            Cst::List(mut list) => {
                list.push(node_box);
                Cst::List(list)
            }
            _ => Cst::List(vec![Box::new(self), node_box]),
        }
    }

    pub fn addlist(self, node: Cst<'c>) -> Cst<'c> {
        let node_box = Box::new(node);
        match self {
            Cst::Nil => Cst::List(vec![node_box]),
            Cst::List(mut list) => {
                list.push(node_box);
                Cst::List(list)
            }
            _ => Cst::List(vec![Box::new(self), node_box]),
        }
    }

    pub fn merge(self, node: Cst<'c>) -> Cst<'c> {
        match (self, node) {
            (Cst::List(mut list), Cst::List(other_list)) => {
                list.extend(other_list);
                Cst::List(list)
            }
            (Cst::List(mut list), other_node) => {
                list.push(Box::new(other_node));
                Cst::List(list)
            }
            (some_node, Cst::List(mut other_list)) => {
                other_list.insert(0, Box::new(some_node));
                Cst::List(other_list)
            }
            (s, n) => s.add(n),
        }
    }

    pub fn closed(self) -> Cst<'c> {
        match self {
            Cst::List(mut list) if list.len() == 1 => *list.pop().unwrap(),
            Cst::List(list) => Cst::Closure(list),
            Cst::Item(cst) => *cst,
            _ => self,
        }
    }

    pub fn distill(self) -> Cst<'c> {
        match self {
            Cst::List(elements) => {
                let mut new_list = Vec::new();
                let mut ast = Ast::new();

                for node in elements {
                    // Recurse to handle nested structures
                    let child = node.distill();
                    
                    match child {
                        Cst::Nil => continue,
                        
                        // Labels are extracted to the current rule's AST
                        Cst::Named(name, value) => {
                            ast.set(&name, *value);
                        }
                        
                        // An Ast from a sub-rule is just another positional item
                        // unless it was Named (handled above).
                        other => new_list.push(Box::new(other)),
                    }
                }

                if !ast.is_empty() {
                    Cst::Ast(Box::new(ast))
                } else {
                    Cst::List(new_list)
                }
            }
            // A standalone Named node always promotes to an Ast
            Cst::Named(name, value) => {
                let mut ast = Ast::new();
                ast.set(&name, *value);
                Cst::Ast(Box::new(ast))
            }
            other => other,
        }
    }
}

pub fn cst_add<'c>(prev: Cst<'c>, node: Cst<'c>) -> Cst<'c> {
    prev.add(node)
}

pub fn cst_addlist<'c>(prev: Cst<'c>, node: Cst<'c>) -> Cst<'c> {
    prev.addlist(node)
}

pub fn cst_merge<'c>(prev: Cst<'c>, node: Cst<'c>) -> Cst<'c> {
    prev.merge(node)
}

pub fn cst_closed(cst: Cst) -> Cst {
    cst.closed()
}