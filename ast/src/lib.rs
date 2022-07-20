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

pub type ApplicationExpr<FX> = sappho_ast_core::ApplicationExpr<Expr<FX>>;
pub type CoreExpr<FX> =
    sappho_ast_core::CoreExpr<Pattern, PureExpr, QueryExpr, ProcExpr, Expr<FX>, FX>;
pub type EffectExpr<FX> = sappho_ast_core::EffectExpr<FX, Expr<FX>>;
pub type FuncDef = sappho_ast_core::FuncDef<Pattern, PureExpr>;
pub type LetClause<FX> = sappho_ast_core::LetClause<Pattern, Expr<FX>>;
pub type LetExpr<FX> = sappho_ast_core::LetExpr<Pattern, Expr<FX>>;
pub type ListExpr<FX> = sappho_listform::ListForm<Expr<FX>, Box<Expr<FX>>>;
pub type LookupExpr<FX> = sappho_ast_core::LookupExpr<Expr<FX>>;
pub type MatchClause<FX> = sappho_ast_core::MatchClause<Pattern, Expr<FX>>;
pub type MatchExpr<FX> = sappho_ast_core::MatchExpr<Pattern, Expr<FX>>;
pub type ObjectDef<FX> =
    sappho_ast_core::ObjectDef<Pattern, PureExpr, QueryExpr, ProcExpr, Expr<FX>>;
pub type ProcExpr = Expr<sappho_ast_core::ProcEffects>;
pub type PureExpr = Expr<sappho_ast_core::PureEffects>;
pub type QueryDef = sappho_ast_core::QueryDef<QueryExpr>;
pub type ProcDef = sappho_ast_core::ProcDef<ProcExpr>;
pub type QueryExpr = Expr<sappho_ast_core::QueryEffects>;
pub use self::expr::Expr;
pub use self::pattern::{ListPattern, Pattern, UnpackPattern};
pub use sappho_ast_core::{Identifier, Literal};
