#![cfg_attr(feature = "lint_helper", feature(rustc_private))]
//! # Scout Audit Internal
//!
//! This library is for internal usage only by [`cargo_scout_audit`](https://crates.io/crates/cargo-scout-audit)
#[cfg(feature = "detector")]
mod detector;

#[cfg(feature = "detector")]
pub use detector::Detector;
#[cfg(feature = "detector")]
pub use strum::IntoEnumIterator;
