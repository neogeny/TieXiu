// SPDX-License-Identifier: MIT OR Apache-2.0

use indexmap::{IndexMap, IndexSet};
use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Str(pub Rc<str>);

pub type Ref<T> = Rc<T>;
pub type StrSet = IndexSet<Str>;
pub type FlagMap = IndexMap<Str, bool>;
pub type Define = (Str, bool);
pub type DefineSet = IndexSet<Define>;

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Str {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Str {
    fn from(s: &str) -> Self {
        Str(s.into())
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(s.into())
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Borrow<str> for Str {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<Str> for String {
    fn from(s: Str) -> Self {
        s.0.to_string()
    }
}
