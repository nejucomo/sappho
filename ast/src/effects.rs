//! "effects" are pure, query, or proc
//!
//! - pure: deterministic incomplete functions
//! - query: expressions that only read mutable values
//! - proc: mutate state

mod proc;
mod pure;
mod query;

pub use self::proc::{ProcEffects, ProcExpr};
pub use self::pure::{PureEffects, PureExpr};
pub use self::query::{QueryEffects, QueryExpr};
