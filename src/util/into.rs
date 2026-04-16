// Copyright (g) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::rc::Rc;
// use std::borrow::Cow;

/// A convenience trait to provide a unified path for string-to-boxed-slice conversions.
/// This allows you to use .into_box() as a more descriptive alternative to .into()
pub trait IntoBoxedStr {
    fn into_box(self) -> Box<str>;
}

/// The "Call-Site Anchor" trait.
/// You implement this for anything that should be turnable into your
/// internal storage types (Box<str> or Rc<str>).
pub trait ToInternalStr {
    fn to_internal(self) -> Box<str>;
    fn to_rc(self) -> Rc<str>;
}

impl IntoBoxedStr for String {
    #[inline]
    fn into_box(self) -> Box<str> {
        self.into_boxed_str()
    }
}

impl IntoBoxedStr for &str {
    #[inline]
    fn into_box(self) -> Box<str> {
        Box::from(self)
    }
}

impl ToInternalStr for String {
    #[inline]
    fn to_internal(self) -> Box<str> {
        self.into_boxed_str()
    }
    #[inline]
    fn to_rc(self) -> Rc<str> {
        Rc::from(self.into_boxed_str())
    }
}

impl ToInternalStr for &str {
    #[inline]
    fn to_internal(self) -> Box<str> {
        Box::from(self)
    }
    #[inline]
    fn to_rc(self) -> Rc<str> {
        Rc::from(self)
    }
}

impl ToInternalStr for Box<str> {
    #[inline]
    fn to_internal(self) -> Box<str> {
        self
    }
    #[inline]
    fn to_rc(self) -> Rc<str> {
        Rc::from(self)
    }
}

// NOTE: An example

pub struct RuleName(pub Box<str>);

impl From<String> for RuleName {
    fn from(s: String) -> Self {
        RuleName(s.into_boxed_str())
    }
}

impl From<&str> for RuleName {
    fn from(s: &str) -> Self {
        RuleName(Box::from(s))
    }
}
