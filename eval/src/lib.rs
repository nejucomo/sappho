mod bind;
mod error;
mod eval;

pub(crate) use self::bind::bind;
pub use self::error::{Error, Result};
pub use self::eval::eval;
pub use sappho_value::ValRef;

#[cfg(test)]
mod tests;
