mod error;
mod eval;
mod scope;
mod value;

pub use self::error::{Error, Result};
pub use self::eval::eval;
pub use self::value::{List, Object, ValRef, Value};

#[cfg(test)]
mod tests;
