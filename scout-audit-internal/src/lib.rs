#![feature(rustc_private)]
//! # Scout Audit Internal
//!
//! This library is for internal usage only by [`cargo_scout_audit`](https://crates.io/crates/cargo-scout-audit)
#[cfg(feature = "detector")]
mod detector;

#[cfg(feature = "lint_helper")]
mod lint_helper;

#[cfg(feature = "detector")]
pub use detector::Detector;
#[cfg(feature = "lint_helper")]
pub use lint_helper::span_lint_and_help;
#[cfg(feature = "detector")]
pub use strum::IntoEnumIterator;
