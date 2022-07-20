//! This Abstract Syntax Tree corresponds to the textual grammar of `sappho`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation, which is embodied in the
//! `sappho-ast-reduced` crate. For example:
//!
//! `fn x -> x` is AST short-hand for reduced `{ fn x -> x }`.
//!
//! The top-level expression for evaluation is [PureExpr], which is a type alias to a general
//! expression type over all effects, [Expr]. The three bespoke effects are [PureEffects],
//! [QueryEffects], and [ProcEffects].

mod expr;
mod pattern;

pub type ApplicationExpr<FX> = sappho_astcore::ApplicationExpr<Expr<FX>>;
pub type CoreExpr<FX> = sappho_astcore::CoreExpr<Pattern, PureExpr, QueryExpr, Expr<FX>, FX>;
pub type EffectExpr<FX> = sappho_astcore::EffectExpr<FX, Expr<FX>>;
pub type FuncDef = sappho_astcore::FuncDef<Pattern, PureExpr>;
pub type LetClause<FX> = sappho_astcore::LetClause<Pattern, Expr<FX>>;
pub type LetExpr<FX> = sappho_astcore::LetExpr<Pattern, Expr<FX>>;
pub type ListExpr<FX> = sappho_listform::ListForm<Expr<FX>, Box<Expr<FX>>>;
pub type LookupExpr<FX> = sappho_astcore::LookupExpr<Expr<FX>>;
pub type MatchClause<FX> = sappho_astcore::MatchClause<Pattern, Expr<FX>>;
pub type MatchExpr<FX> = sappho_astcore::MatchExpr<Pattern, Expr<FX>>;
pub type ObjectDef<FX> = sappho_astcore::ObjectDef<Pattern, PureExpr, QueryExpr, Expr<FX>>;
pub type ProcExpr = Expr<sappho_astcore::ProcEffects>;
pub type PureExpr = Expr<sappho_astcore::PureEffects>;
pub type QueryDef = sappho_astcore::QueryDef<QueryExpr>;
pub type QueryExpr = Expr<sappho_astcore::QueryEffects>;
pub use self::expr::Expr;
pub use self::pattern::{ListPattern, Pattern, UnpackPattern};
pub use sappho_astcore::{Identifier, Literal};
