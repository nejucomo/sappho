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
mod pattern;

pub use sappho_gast::{Identifier, Literal};
pub type ApplicationExpr<FX> = sappho_gast::ApplicationExpr<GenExpr<FX>>;
pub type LetExpr<FX> = sappho_gast::LetExpr<Pattern, GenExpr<FX>>;
pub type LetClause<FX> = sappho_gast::LetClause<Pattern, GenExpr<FX>>;
pub type LookupExpr<FX> = sappho_gast::LookupExpr<GenExpr<FX>>;
pub type MatchExpr<FX> = sappho_gast::MatchExpr<Pattern, GenExpr<FX>>;
pub type MatchClause<FX> = sappho_gast::MatchClause<Pattern, GenExpr<FX>>;
pub type QueryDef = sappho_gast::QueryDef<QueryExpr>;
pub type FuncDef = sappho_gast::FuncDef<Pattern, PureExpr>;
pub type ObjectDef<FX> = sappho_gast::ObjectDef<Pattern, PureExpr, QueryExpr, GenExpr<FX>>;
pub type ListExpr<FX> = sappho_listform::ListForm<GenExpr<FX>, Box<GenExpr<FX>>>;

pub use self::effects::{ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr};
pub use self::expr::GenExpr;
pub use self::pattern::{ListPattern, Pattern, UnpackPattern};
