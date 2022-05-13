mod error;
mod eval;
mod scope;
mod value;

pub use self::error::{Error, Result};
pub use self::eval::eval;
pub use self::value::{ValRef, Value};

#[cfg(test)]
mod tests;
