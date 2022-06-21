//! Low level evaluation of Abstract-Syntax-Tree (AST) expressions via [eval()].
//!
//! For a high-level parse-and-eval functionality, use `sappho-interpreter`.

mod bind;
mod error;
mod eval;
mod expr;
mod traits;

pub use self::error::{Error, Result};
pub use self::eval::eval;

/// A gc-aware reference to a value. See [sappho_value] crate for more detail.
pub use sappho_value::ValRef;

pub(crate) use self::bind::bind;
pub(crate) use self::traits::{Eval, EvalV};
