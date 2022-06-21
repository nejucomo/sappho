mod bind;
mod error;
mod eval;
mod expr;
mod traits;

pub use self::error::{Error, Result};
pub use self::eval::eval;
pub use sappho_value::ValRef;

pub(crate) use self::bind::bind;
pub(crate) use self::traits::{Eval, EvalV};
