mod error;
mod locals;
mod objectval;
mod scope;
mod scoped;
mod valuable;
mod value;

pub use crate::error::{VResult, ValueError};
pub use crate::locals::{BindError, BindErrorReason, Locals};
pub use crate::objectval::{FuncVal, ObjectVal, ProcVal, QueryVal};
pub use crate::scope::{LookupError, LookupErrorReason, Scope};
pub use crate::scoped::Scoped;
pub use crate::valuable::{AsError, Valuable};
pub use crate::value::Value;
