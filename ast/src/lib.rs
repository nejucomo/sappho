//! This Abstract Syntax Tree corresponds to the textual grammar of `sappho`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation, which is embodied in the
//! `sappho-east` (aka "eval ast") crate. For example:
//!
//! `fn x -> x` is AST short-hand for EAST `{ fn x -> x }`.
//!
//! The top-level expression for evaluation is [PureExpr], which is a type alias to a general
//! expression type over all effects, [Expr]. The three bespoke effects are [PureEffects],
//! [QueryEffects], and [ProcEffects].

mod expr;
mod pattern;

pub type ApplicationExpr<FX> = sappho_gast::ApplicationExpr<Expr<FX>>;
pub type CoreExpr<FX> = sappho_gast::CoreExpr<Pattern, PureExpr, QueryExpr, Expr<FX>, FX>;
pub type EffectExpr<FX> = sappho_gast::EffectExpr<FX, Expr<FX>>;
pub type FuncDef = sappho_gast::FuncDef<Pattern, PureExpr>;
pub type LetClause<FX> = sappho_gast::LetClause<Pattern, Expr<FX>>;
pub type LetExpr<FX> = sappho_gast::LetExpr<Pattern, Expr<FX>>;
pub type ListExpr<FX> = sappho_listform::ListForm<Expr<FX>, Box<Expr<FX>>>;
pub type LookupExpr<FX> = sappho_gast::LookupExpr<Expr<FX>>;
pub type MatchClause<FX> = sappho_gast::MatchClause<Pattern, Expr<FX>>;
pub type MatchExpr<FX> = sappho_gast::MatchExpr<Pattern, Expr<FX>>;
pub type ObjectDef<FX> = sappho_gast::ObjectDef<Pattern, PureExpr, QueryExpr, Expr<FX>>;
pub type ProcExpr = Expr<sappho_gast::ProcEffects>;
pub type PureExpr = Expr<sappho_gast::PureEffects>;
pub type QueryDef = sappho_gast::QueryDef<QueryExpr>;
pub type QueryExpr = Expr<sappho_gast::QueryEffects>;
pub use self::expr::Expr;
pub use self::pattern::{ListPattern, Pattern, UnpackPattern};
pub use sappho_gast::{Identifier, Literal};
