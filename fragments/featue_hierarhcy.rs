// Priority 1: PCRE2 (The heavy hitter)
#[cfg(feature = "pcre2")]
mod pcre2;

// Priority 2: Fancy (The fallback)
// Only compiles if fancy is ON AND pcre2 is OFF
#[cfg(all(feature = "fancy", not(feature = "pcre2")))]
mod fancy;

// Re-export logic
#[cfg(feature = "pcre2")]
pub use pcre2::*;

#[cfg(all(feature = "fancy", not(feature = "pcre2")))]
pub use fancy::*;
