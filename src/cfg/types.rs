// SPDX-License-Identifier: MIT OR Apache-2.0

use indexmap::{IndexMap, IndexSet};
use std::rc::Rc;

pub type Ref<T> = Box<T>;
pub type Str = Rc<str>;
pub type StrSet = IndexSet<Str>;
pub type FlagMap = IndexMap<Str, bool>;
pub type Define = (Str, bool);
pub type DefineSet = IndexSet<Define>;
