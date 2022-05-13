mod error;
mod eval;
mod value;

pub use self::error::{Error, Result};
pub use self::eval::eval;
pub use self::value::Value;

#[cfg(test)]
mod tests;
