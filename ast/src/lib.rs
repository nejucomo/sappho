//! This Abstract Syntax Tree corresponds to the textual grammar of `sappho`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation, which is embodied in the
//! `sappho-east` (aka "eval ast") crate. For example:
//!
//! `fn x -> x` is AST short-hand for EAST `{ fn x -> x }`.
//!
//! The top-level expression for evaluation is [PureExpr], which is a type alias to a general
//! expression type over all effects, [GenExpr]. The three bespoke effects are [PureEffects],
//! [QueryEffects], and [ProcEffects].

mod effects;
mod expr;
mod func;
mod letexpr;
mod listform;
mod literal;
mod lookup;
mod matchexpr;
mod object;
mod pattern;
mod query;

/// An identifier such as the name of the argument and reference in `fn x -> x`.
pub type Identifier = sappho_identmap::Identifier;
pub type ApplicationExpr<FX> = sappho_gast::ApplicationExpr<GenExpr<FX>>;

pub use self::effects::{ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr};
pub use self::expr::GenExpr;
pub use self::func::FuncDef;
pub use self::letexpr::{LetClause, LetExpr};
pub use self::listform::ListForm;
pub use self::literal::Literal;
pub use self::lookup::LookupExpr;
pub use self::matchexpr::{MatchClause, MatchExpr};
pub use self::object::ObjectDef;
pub use self::pattern::{Pattern, UnpackPattern};
pub use self::query::QueryDef;
