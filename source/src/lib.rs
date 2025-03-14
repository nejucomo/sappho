//! A crate for loading source code while tracking its provenance.

#![deny(missing_docs)]

mod code;
mod link;
mod scref;
mod source;

pub use self::code::SourceCode;
pub use self::link::SourceCodeLink;
pub use self::scref::{SourceCodeRef, Span};
pub use self::source::Source;
