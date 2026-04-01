// The Generic "Model" that lives in TieXiu
// This stays agnostic of the specific language being parsed.
#[derive(Debug, Deserialize)]
#[serde(tag = "__class__")] // This handles the magic dispatch!
pub enum ModelNode {
    Grammar {
        name: String,
        rules: Vec<ModelNode>
    },
    Rule {
        name: String,
        exp: Box<ModelNode>,
        is_lrec: bool,
        is_memo: bool,
        params: Vec<String>,
    },
    Sequence {
        sequence: Vec<ModelNode>
    },
    Choice {
        options: Vec<ModelNode>
    },
    Token {
        token: String
    },
    Call {
        name: String
    },
    Cut,
    EOF,
    // ... etc for Pattern, Named, Override
}
