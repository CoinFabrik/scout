#![feature(const_trait_impl)]
#![cfg_attr(feature = "lint_helper", feature(rustc_private))]
//! # Scout Audit Internal
//!
//! This library is for internal usage only by [`cargo_scout_audit`](https://crates.io/crates/cargo-scout-audit)
#[cfg(feature = "detector")]
mod detector;
#[cfg(feature = "detector")]
mod socket;

#[cfg(feature = "detector")]
pub use detector::DetectorImpl;
#[cfg(feature = "detector")]
pub use detector::InkDetector;
#[cfg(feature = "detector")]
pub use detector::SorobanDetector;
#[cfg(feature = "detector")]
pub use strum::IntoEnumIterator;
#[cfg(feature = "detector")]
pub use socket::*;
#[cfg(feature = "detector")]
pub use detector::*;

