//! A crate for loading source code while tracking its provenance.

#![deny(missing_docs)]

mod code;
mod link;
mod source;
mod sourced;

pub use self::code::SourceCode;
pub use self::link::SourceCodeLink;
pub use self::source::Source;
pub use self::sourced::{Sourced, Span};
