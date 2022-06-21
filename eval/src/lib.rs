mod bind;
mod error;
mod eval;
mod query;
mod scope;

pub(crate) use self::bind::bind;
pub use self::error::{Error, Result};
pub use self::eval::eval;

#[cfg(test)]
mod tests;
