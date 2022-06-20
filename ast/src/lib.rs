//! This Abstract Syntax Tree corresponds to the textual grammar of `sappho`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation. Example:
//!
//! `fn x -> x` is short-hand for `{ fn x -> x }`.

mod application;
mod effects;
mod expr;
mod func;
mod letexpr;
mod listform;
mod literal;
mod lookup;
mod object;
mod query;

pub type Pattern = Identifier;

pub use self::application::Application;
pub use self::effects::{ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr};
pub use self::expr::GenExpr;
pub use self::func::FuncDef;
pub use self::letexpr::LetExpr;
pub use self::listform::ListForm;
pub use self::literal::Literal;
pub use self::lookup::Lookup;
pub use self::object::ObjectDef;
pub use self::query::QueryDef;
pub use sappho_identmap::Identifier;
