use serde::{Deserialize, Serialize};

// The Generic "Model" that lives in TieXiu
// Agnostic of the specific language, used as the JSON exchange format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__class__")]
pub enum ModelNode {
    Grammar {
        name: String,
        rules: Vec<ModelNode>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        directives: std::collections::HashMap<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        keywords: Vec<String>,
    },
    Rule {
        name: String,
        exp: Box<ModelNode>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_lrec: bool,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_memo: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        params: Vec<String>,
    },
    Sequence {
        sequence: Vec<ModelNode>,
    },
    Choice {
        options: Vec<ModelNode>,
    },
    Option {
        exp: Box<ModelNode>,
    },
    Group {
        exp: Box<ModelNode>,
    },
    Token {
        token: String,
    },
    Pattern {
        pattern: String,
    },
    Constant {
        value: serde_json::Value,
    },
    Call {
        name: String,
    },
    Void {
        exp: Box<ModelNode>,
    },
    Cut,
    EOF,
    Optional {
        exp: Box<ModelNode>,
    },
    Closure {
        exp: Box<ModelNode>,
    },
    PositiveClosure {
        exp: Box<ModelNode>,
    },
    Gather {
        exp: Box<ModelNode>,
        joiner: Box<ModelNode>,
    },
    PositiveGather {
        exp: Box<ModelNode>,
        joiner: Box<ModelNode>,
    },
    LeftJoin {
        exp: Box<ModelNode>,
        joiner: Box<ModelNode>,
    },
    RightJoin {
        exp: Box<ModelNode>,
        joiner: Box<ModelNode>,
    },
    PositiveLookahead {
        exp: Box<ModelNode>,
    },
    NegativeLookahead {
        exp: Box<ModelNode>,
    },
    Named {
        name: String,
        exp: Box<ModelNode>,
    },
    NamedList {
        name: String,
        exp: Box<ModelNode>,
    },
    Override {
        exp: Box<ModelNode>,
    },
    SkipTo {
        exp: Box<ModelNode>,
    },
}
