// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tatsu::TatSuModel;
use crate::model::elements::{ERef, Element};
use crate::model::grammar::Grammar;
use crate::model::rule::{Rule, RuleMap};

impl From<TatSuModel> for ERef {
    fn from(model: TatSuModel) -> Self {
        ERef::new(Element::from(model))
    }
}

impl Grammar {
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // // let value: serde_json::Value = serde_json::from_str(json).map_err(|e| {
        // //     format!("Invalid JSON syntax at line {}, col {}: {}", e.line(), e.column(), e)
        // // })?;
        //
        // // // Debug: If you suspect the JSON structure is wrong, you can inspect 'value' here.
        // // println!("DEBUG: Raw JSON Root Type: {:?}", value.as_object().map(|_| "Object").unwrap_or("Other"));
        // println!("DEBUG: Raw JSON Root Type: {:#}", value);

        // Use a Deserializer to track the path to the error
        let mut deserializer = serde_json::Deserializer::from_str(json);

        let model: TatSuModel =
            serde_path_to_error::deserialize(&mut deserializer).map_err(|err| {
                // This provides the path (e.g., "rules[0].name") and the error message
                format!("JSON error at {}: {}", err.path(), err)
            })?;

        let grammar = Self::try_from(model)?;
        Ok(grammar)
    }
}

impl TryFrom<TatSuModel> for Grammar {
    type Error = String;

    fn try_from(model: TatSuModel) -> Result<Self, Self::Error> {
        if let TatSuModel::Grammar { name, rules, .. } = model {
            let mut rule_vec= vec![];
            let mut registry = RuleMap::new();

            for rule_node in rules {
                if let TatSuModel::Rule {
                    name: r_name, exp, ..
                } = rule_node
                {
                    // Convert the 'dumb' expression to a 'smart' Element
                    let engine_exp: Element = (*exp).into();

                    let rule = Rule::new(&r_name, engine_exp);
                    registry.insert(r_name, rule.clone());
                    rule_vec.push(rule);
                }
            }

            Ok(Grammar {
                name,
                rules: rule_vec.into(),
                rulemap: registry,
            })
        } else {
            Err("Root node must be a Grammar".into())
        }
    }
}

impl From<TatSuModel> for Element {
    fn from(model: TatSuModel) -> Self {
        match model {
            TatSuModel::Grammar { .. } => {
                unreachable!("Container types (Rule/Grammar) cannot be nested inside expressions.");
            }
            TatSuModel::Rule { .. } => Element::Void,
            TatSuModel::RuleInclude { .. } => Element::Void,
            TatSuModel::LeftJoin { .. } => unreachable!("LeftJoin not implemented"),
            TatSuModel::RightJoin { .. } => unreachable!("RightJoin not implemented"),

            // --- Core Terminals ---
            TatSuModel::Cut => Element::Cut,
            TatSuModel::EOF => Element::Eof,
            TatSuModel::Void { .. } => Element::Void,

            // --- Calls and Tokens ---
            TatSuModel::Call { name } => Element::Call(name.into()),
            TatSuModel::Token { token } => Element::Token(token.into()),
            TatSuModel::Pattern { pattern } => Element::Pattern(pattern.into()),
            TatSuModel::Constant { literal } => Element::Constant(literal.to_string().into()),
            TatSuModel::Alert { literal, level } => {
                Element::Alert(literal.to_string().into(), level)
            }

            // --- Unary Operators ---
            TatSuModel::Group { exp } => Element::Group((*exp).into()),
            TatSuModel::Optional { exp } => {
                Element::Optional((*exp).into())
            },
            TatSuModel::Option { exp } => {
                Element::Alt((*exp).into())
            }
            TatSuModel::Closure { exp } => Element::Closure((*exp).into()),
            TatSuModel::PositiveClosure { exp } => Element::PositiveClosure((*exp).into()),

            // --- Lookahead ---
            TatSuModel::Lookahead { exp } => Element::Lookahead((*exp).into()),
            TatSuModel::NegativeLookahead { exp } => Element::NegativeLookahead((*exp).into()),
            TatSuModel::SkipTo { exp } => Element::SkipTo((*exp).into()),

            // --- N-ary Operators ---
            TatSuModel::Sequence { sequence } => {
                let exprs: Vec<Element> = sequence.into_iter().map(|m| m.into()).collect();
                Element::Sequence(exprs.as_slice().into())
            }
            TatSuModel::Choice { options } => {
                let exprs: Vec<Element> = options.into_iter().map(|m| m.into()).collect();
                Element::Choice(exprs.as_slice().into())
            }

            // --- Joins and Gathers ---
            TatSuModel::Join { exp, sep } => Element::Join {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::PositiveJoin { exp, sep } => Element::PositiveJoin {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::Gather { exp, sep } => Element::Gather {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::PositiveGather { exp, sep } => Element::PositiveGather {
                exp: (*exp).into(),
                sep: (*sep).into(),
            },
            TatSuModel::Named { name, exp } => Element::Named(name.into(), (*exp).into()),
            TatSuModel::NamedList { name, exp } => Element::NamedList(name.into(), (*exp).into()),
            TatSuModel::Override { exp } => Element::Override((*exp).into()),
            TatSuModel::OverrideList { exp } => Element::OverrideList((*exp).into()),
            TatSuModel::SkipGroup { exp } => Element::SkipGroup((*exp).into()),
        }
    }
}
