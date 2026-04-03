use crate::astree::{Ast, Cst, KeyValue};
use std::fmt;

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Collect and sort keys for a stable, predictable string
        let mut keys: Vec<&String> = self.fields.keys().collect();
        keys.sort();

        write!(f, "{{")?;
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            // Safe to unwrap because we just got the key from the map
            write!(f, "{}: {}", key, self.fields.get(*key).unwrap())?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "«{}={}»", self.0, self.1)
    }
}

impl fmt::Display for Cst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Token(s) | Self::Literal(s) => write!(f, "\"{}\"", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::List(items) | Self::Closure(items) => {
                let bracket = if matches!(self, Self::List(_)) {
                    ("[", "]")
                } else {
                    ("{", "}")
                };
                write!(f, "{}", bracket.0)?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "{}", bracket.1)
            }
            Self::Named(kv) | Self::NamedList(kv) => write!(f, "{}", kv),
            Self::OverrideValue(v) => write!(f, "!{}", v),
            Self::OverrideList(v) => write!(f, "!!{}", v),
            Self::Ast(ast) => write!(f, "{}", ast),
            Self::Void => write!(f, "()"),
            Self::Nil => write!(f, "∅"),
            Self::Bottom => write!(f, "⊥"),
        }
    }
}
