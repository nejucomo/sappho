//! A crate for loading source code while tracking its provenance.

mod load;
mod source;

pub use self::load::LoadSource;
pub use self::source::Source;
