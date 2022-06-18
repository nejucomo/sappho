mod error;
mod eval;
mod list;
mod object;
mod scope;
mod value;

pub use self::error::{Error, Result};
pub use self::eval::eval;
pub use self::list::List;
pub use self::object::Object;
pub use self::value::{ValRef, Value};

#[cfg(test)]
mod tests;
