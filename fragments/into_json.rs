// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::tatsu::TatSuModel;
use crate::json::Json;
use crate::peg::exp::{ERef, Exp, ParserExp};
use crate::peg::grammar::Grammar;
use crate::peg::rule::{Rule, RuleInfo};
use std::collections::HashMap;

impl From<TatSuModel> for ERef {
    fn from(model: TatSuModel) -> Self {
        ERef::new(Exp::from(model))
    }
}

impl Grammar {
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut deserializer = serde_json::Deserializer::from_str(json);

        let model: TatSuModel =
            serde_path_to_error::deserialize(&mut deserializer).map_err(|err| {
                format!("JSON error at {}: {}", err.path(), err)
            })?;

        let grammar = Self::try_from(model)?;
        Ok(grammar)
    }
}

impl TryFrom<TatSuModel> for Grammar {
    type Error = String;

    fn try_from(model: TatSuModel) -> Result<Self, Self::Error> {
        if let TatSuModel::Grammar {
            name,
            rules,
            directives,
            keywords,
            analyzed,
        } = model
        {
            let mut rule_vec: Vec<Rule> = vec![];
            for rule_model in rules {
                if let TatSuModel::Rule {
                    name,
                    params,
                    exp,
                    is_name,
                    is_tokn,
                    no_memo,
                    is_memo,
                    is_lrec,
                } = rule_model
                {
                    let rhs: Exp = (*exp).into();
                    let rule = Rule {
                        info: RuleInfo {
                            name: name.into(),
                            params: params.iter().map(|p| p.as_str().into()).collect(),
                        }
                        .into(),
                        exp: rhs,
                        is_name,
                        is_tokn,
                        no_memo,
                        is_memo: is_memo && !no_memo,
                        is_lrec,
                    };
                    rule_vec.push(rule);
                }
            }
            let str_directives: HashMap<String, String> = directives
                .iter()
                .map(|(k, v)| {
                    let val_str = v.as_str().map(|s| s.to_string()).unwrap_or(v.to_string());
                    (k.clone(), val_str)
                })
                .collect();
            let mut grammar = Grammar {
                name: name.as_str().into(),
                analyzed,
                rulemap: Grammar::new_rulemap(&rule_vec),
                directives: str_directives,
                keywords,
            };
            grammar.initialize();
            Ok(grammar)
        } else {
            Err("Root node must be a Grammar".into())
        }
    }
}

impl TryFrom<Grammar> for TatSuModel {
    type Error = String;

    fn try_from(grammar: Grammar) -> Result<Self, Self::Error> {
        let rules: Vec<TatSuModel> = grammar
            .rulemap
            .values()
            .map(|r| {
                let rule = r.borrow();
                TatSuModel::Rule {
                    name: rule.info.name.as_str().into(),
                    params: rule.info.params.iter().map(|p| p.as_str().into()).collect(),
                    exp: Box::new(TatSuModel::from(rule.exp.clone())),
                    is_name: rule.is_name,
                    is_tokn: rule.is_tokn,
                    no_memo: rule.no_memo,
                    is_memo: rule.is_memo,
                    is_lrec: rule.is_lrec,
                }
            })
            .collect();

        let directives: HashMap<String, Json> = grammar
            .directives
            .iter()
            .map(|(k, v)| (k.clone(), Json::String(v.clone())))
            .collect();

        Ok(TatSuModel::Grammar {
            name: grammar.name.as_str().into(),
            rules,
            directives,
            keywords: grammar.keywords,
            analyzed: grammar.analyzed,
        })
    }
}

impl From<Exp> for TatSuModel {
    fn from(exp: Exp) -> Self {
        match exp.exp {
            ParserExp::Nil => TatSuModel::Void {
                name: "nil".to_string(),
            },
            ParserExp::Cut => TatSuModel::Cut,
            ParserExp::Void => TatSuModel::Void {
                name: "void".to_string(),
            },
            ParserExp::Eof => TatSuModel::EOF,
            ParserExp::Dot => TatSuModel::Pattern {
                pattern: ".".to_string(),
            },
            ParserExp::Call(name, _) => TatSuModel::Call {
                name: name.as_str().into(),
                params: vec![],
            },
            ParserExp::Token(s) => TatSuModel::Token {
                token: s.as_str().into(),
            },
            ParserExp::Pattern(s) => TatSuModel::Pattern {
                pattern: s.as_str().into(),
            },
            ParserExp::Constant(s) => TatSuModel::Constant {
                literal: Json::String(s.as_str().into()),
            },
            ParserExp::Alert(s, level) => TatSuModel::Alert {
                literal: Json::String(s.as_str().into()),
                level,
            },
            ParserExp::Named(name, eref) => TatSuModel::Named {
                name: name.as_str().into(),
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::NamedList(name, eref) => TatSuModel::NamedList {
                name: name.as_str().into(),
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::Override(eref) => TatSuModel::Override {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::OverrideList(eref) => TatSuModel::OverrideList {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::Group(eref) => TatSuModel::Group {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::SkipGroup(eref) => TatSuModel::SkipGroup {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::Lookahead(eref) => TatSuModel::Lookahead {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::NegativeLookahead(eref) => TatSuModel::NegativeLookahead {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::SkipTo(eref) => TatSuModel::SkipTo {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::Sequence(refs) => TatSuModel::Sequence {
                sequence: refs.iter().map(|r| TatSuModel::from(r.as_exp())).collect(),
            },
            ParserExp::Choice(refs) => TatSuModel::Choice {
                options: refs.iter().map(|r| TatSuModel::from(r.as_exp())).collect(),
            },
            ParserExp::Alt(eref) => TatSuModel::Option {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::Optional(eref) => TatSuModel::Optional {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::Closure(eref) => TatSuModel::Closure {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::PositiveClosure(eref) => TatSuModel::PositiveClosure {
                exp: Box::new(TatSuModel::from(eref.as_exp())),
            },
            ParserExp::Join { exp, sep } => TatSuModel::Join {
                exp: Box::new(TatSuModel::from(exp.as_exp())),
                sep: Box::new(TatSuModel::from(sep.as_exp())),
            },
            ParserExp::PositiveJoin { exp, sep } => TatSuModel::PositiveJoin {
                exp: Box::new(TatSuModel::from(exp.as_exp())),
                sep: Box::new(TatSuModel::from(sep.as_exp())),
            },
            ParserExp::Gather { exp, sep } => TatSuModel::Gather {
                exp: Box::new(TatSuModel::from(exp.as_exp())),
                sep: Box::new(TatSuModel::from(sep.as_exp())),
            },
            ParserExp::PositiveGather { exp, sep } => TatSuModel::PositiveGather {
                exp: Box::new(TatSuModel::from(exp.as_exp())),
                sep: Box::new(TatSuModel::from(sep.as_exp())),
            },
            ParserExp::RuleInclude { name, exp } => TatSuModel::RuleInclude {
                name: name.as_str().into(),
                exp: Box::new(TatSuModel::from(exp.as_exp())),
            },
            _ => unreachable!("Conversion for variant not implemented"),
        }
    }
}

impl From<TatSuModel> for Exp {
    fn from(model: TatSuModel) -> Self {
        match model {
            TatSuModel::Grammar { .. } | TatSuModel::Rule { .. } => {
                unreachable!("Container types cannot be nested inside expressions.");
            }
            TatSuModel::RuleInclude { name, exp } => Exp::rule_include(&name, (*exp).into()),
            TatSuModel::LeftJoin { .. } | TatSuModel::RightJoin { .. } => {
                unreachable!("Join types not implemented")
            }
            TatSuModel::Cut => Exp::cut(),
            TatSuModel::EOF => Exp::eof(),
            TatSuModel::Void { .. } => Exp::void(),
            TatSuModel::Call { name, .. } => Exp::call(name.as_str(), Exp::nil()),
            TatSuModel::Token { token } => Exp::token(token.as_str()),
            TatSuModel::Pattern { pattern } => Exp::pattern(pattern.as_str()),
            TatSuModel::Constant { literal } => Exp::constant(literal.as_str().unwrap_or("")),
            TatSuModel::Alert { literal, level } => Exp::alert(literal.as_str().unwrap(), level),
            TatSuModel::Group { exp } => Exp::group((*exp).into()),
            TatSuModel::Optional { exp } => Exp::optional((*exp).into()),
            TatSuModel::Option { exp } => Exp::alt((*exp).into()),
            TatSuModel::Closure { exp } => Exp::closure((*exp).into()),
            TatSuModel::PositiveClosure { exp } => Exp::positive_closure((*exp).into()),
            TatSuModel::Lookahead { exp } => Exp::lookahead((*exp).into()),
            TatSuModel::NegativeLookahead { exp } => Exp::negative_lookahead((*exp).into()),
            TatSuModel::SkipTo { exp } => Exp::skip_to((*exp).into()),
            TatSuModel::Sequence { sequence } => {
                let exprs: Vec<Exp> = sequence.into_iter().map(|m| m.into()).collect();
                Exp::sequence(exprs.as_slice().into())
            }
            TatSuModel::Choice { options } => {
                let exprs: Vec<Exp> = options.into_iter().map(|m| m.into()).collect();
                Exp::choice(exprs.as_slice().into())
            }
            TatSuModel::Join { exp, sep } => Exp::join((*exp).into(), (*sep).into()),
            TatSuModel::PositiveJoin { exp, sep } => {
                Exp::positive_join((*exp).into(), (*sep).into())
            }
            TatSuModel::Gather { exp, sep } => Exp::gather((*exp).into(), (*sep).into()),
            TatSuModel::PositiveGather { exp, sep } => {
                Exp::positive_gather((*exp).into(), (*sep).into())
            }
            TatSuModel::Named { name, exp } => Exp::named(name.as_str(), (*exp).into()),
            TatSuModel::NamedList { name, exp } => Exp::named_list(name.as_str(), (*exp).into()),
            TatSuModel::Override { exp } => Exp::override_node((*exp).into()),
            TatSuModel::OverrideList { exp } => Exp::override_list((*exp).into()),
            TatSuModel::SkipGroup { exp } => Exp::skip_group((*exp).into()),
        }
    }
}
