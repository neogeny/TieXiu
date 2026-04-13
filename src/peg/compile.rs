// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Grammar, Rule};
use crate::trees::{NodeMeta, Tree, TreeMap};
use indexmap::IndexMap;
use thiserror::Error;

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CompileError {
    #[error("expected {0} to be a Tree::Node")]
    ExpectedNode(String),

    #[error("expected {0} to contain a Tree::Map")]
    ExpectedMap(String),

    #[error("expected {0} to be Tree::Text")]
    ExpectedText(&'static str),

    #[error("expected {0} to be Tree::List")]
    ExpectedList(String),

    #[error("expected {0} to be Tree::List or Tree::Nil")]
    ExpectedListOrNil(&'static str),

    #[error("expected {0} to be Tree::Text or Tree::Nil")]
    ExpectedTextOrNil(&'static str),

    #[error("expected {context} to contain key '{key}'")]
    MissingKey {
        context: &'static str,
        key: &'static str,
    },

    #[error("expected {0}")]
    ExpectedField(&'static str),

    #[error("expected {expected}, found '{found}'")]
    UnexpectedNodeName {
        expected: &'static str,
        found: Box<str>,
    },

    #[error("expected {expected}, found '{found}'")]
    UnexpectedTypeName { expected: Box<str>, found: Box<str> },

    #[error("{0} is not implemented")]
    NotImplemented(&'static str),

    #[error("compile_rhs() does not support node '{0}'")]
    UnsupportedRhs(Box<str>),
}

#[derive(Debug, Default)]
pub struct GrammarCompiler {
    rulemap: IndexMap<Box<str>, Rule>,
}

fn parse_node(node: &Tree) -> CompileResult<(&NodeMeta, &Tree)> {
    let Tree::Node { meta, tree } = node else {
        return Err(CompileError::ExpectedNode(format!("{:?}", node)));
    };
    Ok((meta, tree))
}

fn parse_node_check<'n>(
    node: &'n Tree,
    name: &'static str,
    typename: &'static str,
) -> CompileResult<(&'n NodeMeta, &'n Tree)> {
    let (meta, tree) = parse_node(node)?;
    if *meta.name != *name {
        return Err(CompileError::UnexpectedNodeName {
            expected: name,
            found: meta.name.clone(),
        });
    }
    if !typename.is_empty() && *meta.params[0] != *typename {
        return Err(CompileError::UnexpectedTypeName {
            expected: typename.into(),
            found: meta.params[0].clone(),
        });
    }
    Ok((meta, tree))
}

fn parse_map(node: &Tree) -> CompileResult<&TreeMap> {
    let Tree::Map(map) = node else {
        return Err(CompileError::ExpectedMap(format!("{:?}", node)));
    };
    Ok(map)
}

fn parse_list(node: &Tree) -> CompileResult<&[Tree]> {
    match node {
        Tree::List(list) | Tree::Closed(list) => Ok(list),
        _ => Err(CompileError::ExpectedList(format!("{:?}", node))),
    }
}

fn map_get<'m>(
    map: &'m TreeMap,
    context: &'static str,
    key: &'static str,
) -> CompileResult<&'m Tree> {
    match map.get(key) {
        Some(node) => Ok(node),
        None => Err(CompileError::MissingKey { context, key }),
    }
}

impl GrammarCompiler {
    pub fn new() -> GrammarCompiler {
        GrammarCompiler {
            rulemap: IndexMap::new(),
        }
    }

    pub fn compile_grammar(&self, tree: &Tree) -> CompileResult<Grammar> {
        // NOTE:
        //  If we get called then the `Tree` is not any generic `Tree` but one
        //  produced by parsing a grammar description written in the TieXiu/TatSu
        //  variant of EBNF they accept.
        //  _
        //  We know the exact structure of the `Tree` so we'll parse it top down
        //  validating the expected node type at each step.
        //  _
        //  All nodes of type `Tree::Node` are produced by a rule in the meta-grammar
        //  so it's possible to dispatch by the rule name in `node.meta.name`.
        //  _
        //  Some `Tree::Node` have an associated node type in `node.meta.params[0]`
        //  and that too can be verified

        let (_, tree) = parse_node_check(tree, "start", "")?;
        let (_meta, tree) = parse_node_check(tree, "grammar", "Grammar")?;
        let map = parse_map(tree)?;
        let rules_node = map_get(map, "grammar", "rules")?;
        // let rules = parse_list(rules_node)?;
        eprintln!("{:?}", map);
        panic!("SEE THE TREE");
        // if *meta.name != *"Grammar" {
        //     return Err(CompileError::UnexpectedNodeName {
        //         expected: "Grammar",
        //         found: meta.into(),
        //     });
        // }
        // self.expect_keys(m, &["name", "directives", "keywords", "rules"], "Grammar")?;
        //
        // let grammar_name = self.text_field(m, "name")?;
        // let directives = self.directives(self.field(m, "directives")?)?;
        // let keywords = self.keywords(self.field(m, "keywords")?)?;
        // let rule_trees = self.list_field(m, "rules")?;
        // let rules = self.compile_rules(rule_trees)?;

        let grammar = Grammar::new("GRAMMAR", &[]);
        // grammar.directives = directives;
        // grammar.keywords = keywords;
        Ok(grammar)
    }
}

impl Grammar {
    pub fn compile(tree: &Tree) -> CompileResult<Self> {
        let compiler = GrammarCompiler::new();
        compiler.compile_grammar(tree)
    }
}
