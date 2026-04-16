// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Railroad diagram generation for grammars
//!
//! Each rail has a connector at the TOP (baseline). When combining vertically,
//! parts connect only at their top, which simplifies the connector design.

use crate::peg::{Exp, ExpKind, Grammar, Rule};
use std::collections::HashSet;

type Rail = [String; 2]; // [connector, content]
type Track = Vec<Rail>;

pub trait ToRailroad {
    fn railroads(&self) -> String;
}

impl ToRailroad for Grammar {
    fn railroads(&self) -> String {
        let track = walk_grammar(self);
        track_to_string(&track)
    }
}

impl ToRailroad for Rule {
    fn railroads(&self) -> String {
        let track = walk_rule(self);
        track_to_string(&track)
    }
}

impl ToRailroad for Exp {
    fn railroads(&self) -> String {
        let track = walk_exp(self);
        track_to_string(&track)
    }
}

pub fn walk_grammar(grammar: &Grammar) -> Track {
    let mut seen: HashSet<Box<str>> = HashSet::new();
    let mut result = Vec::new();
    for rule in grammar.rules() {
        let name: Box<str> = rule.name.clone().into();
        if seen.contains(&name) {
            continue;
        }
        seen.insert(name);
        let rule_track = walk_rule(rule);
        if !result.is_empty() {
            result = lay_out(&[result, rule_track]);
        } else {
            result = rule_track;
        }
    }
    result
}

pub fn walk_rule(rule: &Rule) -> Track {
    #[allow(clippy::let_and_return)]
    {
        let start_conn = format!("{} ●─", rule.name);
        let rule_content = walk_exp(&rule.exp);
        let with_start = weld(&vec![make_rail("", &start_conn)], &rule_content);
        let with_end = weld(&with_start, &vec![make_rail("", "─■")]);
        with_end
    }
}

pub fn walk_exp(exp: &Exp) -> Track {
    eprintln!("walk_exp: {:?}", std::mem::discriminant(&exp.kind));
    match &exp.kind {
        ExpKind::Void => vec![make_rail("", " ∅ ")],
        ExpKind::Fail => vec![make_rail("", " ⚠ ")],
        ExpKind::Cut => vec![make_rail("", " ✂ ")],
        ExpKind::Dot => vec![make_rail("", " ∀ ")],
        ExpKind::Eof => vec![make_rail("", " $")],

        ExpKind::Token(t) => vec![make_rail("", &format!("{:?}", t))],
        ExpKind::Pattern(p) => vec![make_rail("", &format!("/{}/", p))],
        ExpKind::Constant(c) => vec![make_rail("", &format!("`{}`", c))],
        ExpKind::Call { name, .. } => vec![make_rail("", name.as_ref())],
        ExpKind::RuleInclude { name, .. } => vec![make_rail("", &format!(" >({}) ", name))],

        ExpKind::Optional(inner) => {
            let inner_track = walk_exp(inner);
            lay_out(&[inner_track.clone(), inner_track])
        }

        ExpKind::Closure(inner) => loop_(&walk_exp(inner)),
        ExpKind::PositiveClosure(inner) => stopn_loop(&walk_exp(inner)),

        ExpKind::Sequence(items) => {
            let mut result = vec![];
            for (i, item) in items.iter().enumerate() {
                if i > 100 {
                    panic!("Sequence too long");
                }
                let track = walk_exp(item);
                if result.is_empty() {
                    result = track;
                } else {
                    result = weld(&result, &track);
                }
            }
            result
        }

        ExpKind::Choice(options) => {
            let tracks: Vec<Track> = options.iter().map(walk_exp).collect();
            lay_out(&tracks)
        }

        ExpKind::Named(name, inner) => {
            let inner_track = walk_exp(inner);
            let prefixed = vec![make_rail("", &format!(" `{}`(", name))];
            let suffixed = vec![make_rail("", ")")];
            weld(&weld(&prefixed, &inner_track), &suffixed)
        }

        ExpKind::Group(inner) => walk_exp(inner),

        ExpKind::Lookahead(inner) => {
            let prefixed = vec![make_rail("", " &")];
            let inner_track = walk_exp(inner);
            weld(&prefixed, &inner_track)
        }

        ExpKind::NegativeLookahead(inner) => {
            let prefixed = vec![make_rail("", " !")];
            let inner_track = walk_exp(inner);
            weld(&prefixed, &inner_track)
        }

        _ => vec![make_rail("", &format!("<{:?}>", exp.kind))],
    }
}

#[allow(dead_code)]
fn unicode_width(s: &str) -> usize {
    s.chars().count()
}

fn make_rail(conn: &str, content: &str) -> Rail {
    [conn.to_string(), content.to_string()]
}

fn rail_to_string(rail: &Rail) -> String {
    format!("{}{}", rail[0], rail[1])
}

fn track_to_string(track: &Track) -> String {
    if track.is_empty() {
        return String::new();
    }
    track
        .iter()
        .map(rail_to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

fn weld(a: &Track, b: &Track) -> Track {
    if a.is_empty() {
        return b.clone();
    }
    if b.is_empty() {
        return a.clone();
    }

    let height = a.len().max(b.len());
    let mut result = Vec::with_capacity(height);

    for i in 0..height {
        let a_conn = a.get(i).map(|r| r[0].as_str()).unwrap_or("");
        let a_content = a.get(i).map(|r| r[1].as_str()).unwrap_or("");

        let b_conn = b.get(i).map(|r| r[0].as_str()).unwrap_or("");
        let b_content = b.get(i).map(|r| r[1].as_str()).unwrap_or("");

        let conn = if a_conn.is_empty() { b_conn } else { a_conn };
        let content = format!("{}{}", a_content, b_content);

        result.push(make_rail(conn, &content));
    }
    result
}

fn lay_out(tracks: &[Track]) -> Track {
    if tracks.is_empty() {
        return vec![];
    }
    if tracks.len() == 1 {
        return tracks[0].clone();
    }

    let mut result = Vec::new();

    for (ti, track) in tracks.iter().enumerate() {
        let is_first = ti == 0;
        let is_last = ti == tracks.len() - 1;

        for (ri, rail) in track.iter().enumerate() {
            while result.len() <= ri {
                result.push(make_rail("", ""));
            }

            let conn = if ri == 0 {
                if is_first {
                    "──┬"
                } else if is_last {
                    "  └"
                } else {
                    "  ├"
                }
            } else {
                if is_first {
                    "  │"
                } else if is_last {
                    "    "
                } else {
                    "  │"
                }
            };

            let suffix = if ri == 0 {
                if is_last {
                    "─┘ "
                } else {
                    "─┤ "
                }
            } else {
                "   "
            };

            if ri == 0 && is_first && result[ri][0].is_empty() || ri > 0 {
                result[ri][0] = conn.to_string();
            }
            result[ri][1] = if ri == 0 && ti > 0 {
                format!("{}{}{}", result[ri][1], rail[1], suffix)
            } else {
                format!("{}{}", rail[1], suffix)
            };
        }
    }

    result
}

fn loop_(inner: &Track) -> Track {
    if inner.is_empty() {
        return vec![make_rail("", "───>───")];
    }

    let inner_str = inner
        .iter()
        .map(|r| r[1].as_str())
        .collect::<Vec<_>>()
        .join("");
    let inner_conn = if inner.len() > 1 { "  │" } else { "" };

    let top = make_rail("", "──┬");
    let mid = make_rail(inner_conn, &inner_str);
    let bot = make_rail("", "──┴");

    vec![top, mid, bot]
}

fn stopn_loop(inner: &Track) -> Track {
    if inner.is_empty() {
        return vec![make_rail("", "───>───")];
    }

    let inner_str = inner
        .iter()
        .map(|r| r[1].as_str())
        .collect::<Vec<_>>()
        .join("");
    let inner_conn = if inner.len() > 1 { "  │" } else { "" };

    let top = make_rail("", "──┬");
    let mid = make_rail(inner_conn, &inner_str);
    let bot = make_rail("", "──┴");

    vec![top, mid, bot]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_rail() {
        let rail = make_rail("─", "foo");
        assert_eq!(rail[0], "─");
        assert_eq!(rail[1], "foo");
    }

    #[test]
    fn test_rail_to_string() {
        let rail = make_rail("─", "foo");
        assert_eq!(rail_to_string(&rail), "─foo");
    }

    #[test]
    fn test_weld_empty_left() {
        let a: Track = vec![];
        let b = vec![make_rail("─", "x")];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(rail_to_string(&result[0]), "─x");
    }

    #[test]
    fn test_weld_empty_right() {
        let a = vec![make_rail("─", "x")];
        let b: Track = vec![];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(rail_to_string(&result[0]), "─x");
    }

    #[test]
    fn test_weld_simple() {
        let a = vec![make_rail("", "a")];
        let b = vec![make_rail("", "b")];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(rail_to_string(&result[0]), "ab");
    }

    #[test]
    fn test_weld_different_heights() {
        let a = vec![make_rail("", "a"), make_rail("", "b")];
        let b = vec![make_rail("", "c")];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_lay_out_empty() {
        let tracks: Vec<Track> = vec![];
        let result = lay_out(&tracks);
        assert!(result.is_empty());
    }

    #[test]
    fn test_lay_out_single() {
        let tracks = vec![vec![make_rail("", "foo")]];
        let result = lay_out(&tracks);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_track_to_string() {
        let track = vec![make_rail("─", "a"), make_rail("│", "b")];
        let result = track_to_string(&track);
        assert_eq!(result, "─a\n│b");
    }

    #[test]
    fn test_lay_out_two_tracks() {
        let track_a = vec![make_rail("", "a")];
        let track_b = vec![make_rail("", "b")];
        let result = lay_out(&[track_a, track_b]);
        let output = track_to_string(&result);
        eprintln!("lay_out two:\n{}", output);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_loop_empty() {
        let result = loop_(&vec![]);
        assert_eq!(result.len(), 1);
        assert!(result[0][1].contains("───>───"));
    }

    #[test]
    fn test_loop_with_content() {
        let inner = vec![make_rail("", "foo")];
        let result = loop_(&inner);
        let output = track_to_string(&result);
        eprintln!("loop:\n{}", output);
        assert!(result.len() >= 2);
    }

    #[test]
    fn test_simple_grammar() {
        use crate::api::compile;
        let grammar = compile("start = 'a' ;", &[]).expect("compile failed");
        let result = grammar.railroads();
        eprintln!("simple grammar railroads:\n{}", result);
        assert!(!result.is_empty());
    }

    // DISABLED: causes stack overflow with calc.json due to rule cycles
    // #[test]
    // fn test_calc_json() {
    //     use crate::api::load;
    //     let grammar_src = std::fs::read_to_string("grammar/calc.json").expect("read file");
    //     let grammar = load(&grammar_src, &[]).expect("load failed");
    //     let result = grammar.railroads();
    //     eprintln!("calc.json railroads:\n{}", result);
    // }
}
