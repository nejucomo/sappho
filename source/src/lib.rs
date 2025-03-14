//! A crate for loading source code while tracking its provenance.

mod code;
mod link;
mod load;
mod source;
mod sourced;

pub use self::code::SourceCode;
pub use self::link::SourceCodeLink;
pub use self::load::LoadSource;
pub use self::source::Source;
pub use self::sourced::{Sourced, Span};
