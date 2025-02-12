//! Provide deterministic mapping from identifiers to values

mod error;
mod identifier;
mod map;
mod tryinto;
mod unroll;

pub use self::error::RedefinitionError;
pub use self::identifier::{IdentRef, Identifier};
pub use self::map::IdentMap;
pub use self::tryinto::TryIntoIdentMap;
pub use self::unroll::ListUnroll;
