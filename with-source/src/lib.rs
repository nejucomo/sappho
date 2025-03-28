//! Provides the [WithSource] type for tracking source with parse outputs
//!
//! This is especially useful for error displays.
#![deny(missing_docs)]

mod parsing;
mod wsource;

pub use crate::wsource::WithSource;
