//! Recursive-subgrammars which carry their effects
//!
//! These have the same effects as the top-level expression type. For example, a list expression in
//! a pure context contains pure expressions, while a list expression in a proc context contains proc
//! expressions.

mod apply;
mod letexpr;
mod lookup;

pub use self::apply::Application;
pub use self::letexpr::LetExpr;
pub use self::lookup::Lookup;
