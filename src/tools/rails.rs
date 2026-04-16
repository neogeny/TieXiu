// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Railroad diagram generation for grammars

use crate::peg::{Exp, ExpKind, Grammar, Rule};
use std::rc::Rc;

type Track = Vec<Rc<str>>;

fn make_rail(content: &str) -> Rc<str> {
    content.into()
}

#[allow(dead_code)]
fn rail_to_string(rail: &Rc<str>) -> String {
    rail.as_ref().to_string()
}

#[allow(dead_code)]
fn track_to_string(track: &Track) -> String {
    if track.is_empty() {
        return String::new();
    }
    track
        .iter()
        .map(|s| s.as_ref().trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn join_tracks(tracks: impl IntoIterator<Item = Track>) -> Track {
    tracks.into_iter().flatten().collect()
}

fn weld(a: &[Rc<str>], b: &[Rc<str>]) -> Track {
    if a.is_empty() {
        return b.to_vec();
    }
    if b.is_empty() {
        return a.to_vec();
    }

    let height = a.len().max(b.len());
    let mut result = Vec::with_capacity(height);

    for i in 0..height {
        let a_content = a.get(i).map(|s| s.as_ref()).unwrap_or("");
        let b_content = b.get(i).map(|s| s.as_ref()).unwrap_or("");
        result.push(make_rail(&format!("{}{}", a_content, b_content)));
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

    let maxl = tracks
        .iter()
        .filter_map(|t| t.first())
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    let mut result = Vec::new();

    for (ti, track) in tracks.iter().enumerate() {
        let is_first = ti == 0;
        let is_last = ti == tracks.len() - 1;

        for (ri, line) in track.iter().enumerate() {
            while result.len() <= ri {
                result.push(make_rail(""));
            }

            let _conn = if ri == 0 {
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

            let padded = pad(line.as_ref(), maxl);
            let new_content = if ri == 0 && ti > 0 {
                format!("{}{}{}", result[ri].as_ref(), padded, suffix)
            } else {
                format!("{}{}", padded, suffix)
            };
            result[ri] = make_rail(&new_content);
        }
    }

    result
}

fn pad(s: &str, width: usize) -> String {
    format!("{}{}", s, " ".repeat(width.saturating_sub(s.len())))
}

fn loop_(inner: &Track) -> Track {
    if inner.is_empty() {
        return vec![make_rail("───>───")];
    }

    let maxl = inner.iter().map(|s| s.len()).max().unwrap_or(0);
    let inner_str = inner
        .iter()
        .map(|s| s.as_ref())
        .collect::<Vec<_>>()
        .join("");
    let inner_conn = if inner.len() > 1 { "  │" } else { "" };

    let top = make_rail("──┬");
    let mid = make_rail(&format!("{}{}", inner_conn, pad(&inner_str, maxl)));
    let bot = make_rail("──┴");

    vec![top, mid, bot]
}

fn stopn_loop(inner: &Track) -> Track {
    if inner.is_empty() {
        return vec![make_rail("───>───")];
    }

    let maxl = inner.iter().map(|s| s.len()).max().unwrap_or(0);
    let inner_str = inner
        .iter()
        .map(|s| s.as_ref())
        .collect::<Vec<_>>()
        .join("");
    let inner_conn = if inner.len() > 1 { "  │" } else { "" };

    let top = make_rail("──┬");
    let mid = make_rail(&format!("{}{}", inner_conn, pad(&inner_str, maxl)));
    let bot = make_rail("──┴");

    vec![top, mid, bot]
}

pub trait ToRailroad {
    fn railroads(&self) -> String;
}

impl ToRailroad for Grammar {
    fn railroads(&self) -> String {
        let tracks: Vec<Track> = self.rules().map(walk_rule).collect();
        let result = join_tracks(tracks);
        track_to_string(&result)
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

pub fn walk_grammar(grammar: &Grammar) -> Vec<Track> {
    grammar.rules().map(walk_rule).collect()
}

pub fn walk_rule(rule: &Rule) -> Track {
    let start_conn = format!("{} ●─", rule.name);
    let rule_content = walk_exp(&rule.exp);
    let with_start = weld(&[make_rail(&start_conn)], &rule_content);
    weld(&with_start, &[make_rail("─■")])
}

fn walk_exp(exp: &Exp) -> Track {
    match &exp.kind {
        ExpKind::Void => vec![make_rail(" ∅ ")],
        ExpKind::Fail => vec![make_rail(" ⚠ ")],
        ExpKind::Cut => vec![make_rail(" ✂ ")],
        ExpKind::Dot => vec![make_rail(" ∀ ")],
        ExpKind::Eof => vec![make_rail(" $")],

        ExpKind::Token(t) => vec![make_rail(&format!("{:?}", t))],
        ExpKind::Pattern(p) => vec![make_rail(&format!("/{}/", p))],
        ExpKind::Constant(c) => vec![make_rail(&format!("`{}`", c))],
        ExpKind::Alert(c, level) => vec![make_rail(&format!(
            "{}^`{}`",
            "^".repeat(*level as usize),
            c
        ))],

        ExpKind::Call { name, .. } => vec![make_rail(name.as_ref())],
        ExpKind::RuleInclude { name, .. } => vec![make_rail(&format!(" >({}) ", name))],

        ExpKind::Optional(inner) => {
            let inner_track = walk_exp(inner);
            lay_out(&[inner_track.clone(), inner_track])
        }

        ExpKind::Closure(inner) => loop_(&walk_exp(inner)),
        ExpKind::PositiveClosure(inner) => stopn_loop(&walk_exp(inner)),

        ExpKind::Join { exp, sep } => {
            let exp_track = walk_exp(exp);
            let sep_track = walk_exp(sep);
            let joined = weld(&sep_track, &[make_rail(" ✂ ─")]);
            let joined = weld(&joined, &exp_track);
            loop_(&joined)
        }
        ExpKind::PositiveJoin { exp, sep } => {
            let exp_track = walk_exp(exp);
            let sep_track = walk_exp(sep);
            let joined = weld(&sep_track, &[make_rail(" ✂ ─")]);
            let joined = weld(&joined, &exp_track);
            stopn_loop(&joined)
        }
        ExpKind::Gather { exp, sep } => {
            let exp_track = walk_exp(exp);
            let sep_track = walk_exp(sep);
            let gathered = weld(&sep_track, &[make_rail(" │ ")]);
            let gathered = weld(&gathered, &exp_track);
            loop_(&gathered)
        }
        ExpKind::PositiveGather { exp, sep } => {
            let exp_track = walk_exp(exp);
            let sep_track = walk_exp(sep);
            let gathered = weld(&sep_track, &[make_rail(" │ ")]);
            let gathered = weld(&gathered, &exp_track);
            stopn_loop(&gathered)
        }

        ExpKind::SkipTo(inner) => {
            let prefixed = vec![make_rail(" ->(")];
            let inner_track = walk_exp(inner);
            weld(&prefixed, &weld(&inner_track, &[make_rail(")")]))
        }

        ExpKind::Sequence(items) => {
            if items.is_empty() {
                vec![make_rail("")]
            } else {
                let mut result = walk_exp(&items[0]);
                for item in &items[1..] {
                    result = weld(&result, &walk_exp(item));
                }
                result
            }
        }

        ExpKind::Choice(options) => {
            let tracks: Vec<Track> = options.iter().map(walk_exp).collect();
            lay_out(&tracks)
        }
        ExpKind::Alt(inner) => walk_exp(inner),

        ExpKind::Named(name, inner) => {
            let inner_track = walk_exp(inner);
            let prefixed = vec![make_rail(&format!(" `{}`(", name))];
            let suffixed = vec![make_rail(")")];
            weld(&weld(&prefixed, &inner_track), &suffixed)
        }

        ExpKind::NamedList(name, inner) => {
            let inner_track = walk_exp(inner);
            let prefixed = vec![make_rail(&format!(" `{}`]+(", name))];
            let suffixed = vec![make_rail(")")];
            weld(&weld(&prefixed, &inner_track), &suffixed)
        }

        ExpKind::Group(inner) => walk_exp(inner),
        ExpKind::SkipGroup(inner) => walk_exp(inner),

        ExpKind::Lookahead(inner) => {
            let prefixed = vec![make_rail(" &")];
            let inner_track = walk_exp(inner);
            weld(&prefixed, &inner_track)
        }

        ExpKind::NegativeLookahead(inner) => {
            let prefixed = vec![make_rail(" !")];
            let inner_track = walk_exp(inner);
            weld(&prefixed, &inner_track)
        }

        ExpKind::Override(inner) => {
            let prefixed = vec![make_rail(r" @(")];
            let inner_track = walk_exp(inner);
            weld(&prefixed, &weld(&inner_track, &[make_rail(")")]))
        }

        ExpKind::OverrideList(inner) => {
            let prefixed = vec![make_rail(r" @+(")];
            let inner_track = walk_exp(inner);
            weld(&prefixed, &weld(&inner_track, &[make_rail(")")]))
        }

        ExpKind::Nil => vec![make_rail("")],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_rail() {
        let rail = make_rail("foo");
        assert_eq!(rail.as_ref(), "foo");
    }

    #[test]
    fn test_rail_to_string() {
        let rail = make_rail("foo");
        assert_eq!(rail_to_string(&rail), "foo");
    }

    #[test]
    fn test_weld_empty_left() {
        let a: Track = vec![];
        let b = vec![make_rail("x")];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_ref(), "x");
    }

    #[test]
    fn test_weld_empty_right() {
        let a = vec![make_rail("x")];
        let b: Track = vec![];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_ref(), "x");
    }

    #[test]
    fn test_weld_simple() {
        let a = vec![make_rail("a")];
        let b = vec![make_rail("b")];
        let result = weld(&a, &b);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_ref(), "ab");
    }

    #[test]
    fn test_weld_different_heights() {
        let a = vec![make_rail("a"), make_rail("b")];
        let b = vec![make_rail("c")];
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
        let tracks = vec![vec![make_rail("foo")]];
        let result = lay_out(&tracks);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_track_to_string() {
        let track = vec![make_rail("a"), make_rail("b")];
        let result = track_to_string(&track);
        assert_eq!(result, "a\nb");
    }

    #[test]
    fn test_lay_out_two_tracks() {
        let track_a = vec![make_rail("a")];
        let track_b = vec![make_rail("b")];
        let result = lay_out(&[track_a, track_b]);
        let output = track_to_string(&result);
        eprintln!("lay_out two:\n{}", output);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_loop_empty() {
        let result = loop_(&vec![]);
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("───>───"));
    }

    #[test]
    fn test_loop_with_content() {
        let inner = vec![make_rail("foo")];
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

    #[test]
    fn test_calc_json() {
        use crate::api::load;
        let grammar_src = std::fs::read_to_string("grammar/calc.json").expect("read file");
        let grammar = load(&grammar_src, &[]).expect("load failed");
        let result = grammar.railroads();
        eprintln!("calc.json railroads:\n{}", result);
    }
}
