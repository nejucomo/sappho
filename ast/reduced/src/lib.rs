//! The Reduced Abstract Syntax Tree is a subset of [`sappho-ast`] for evaluation

mod expr;
mod pattern;

pub type ApplicationExpr<FX> = sappho_ast_core::ApplicationExpr<Expr<FX>>;
pub type CoreExpr<FX> =
    sappho_ast_core::CoreExpr<Pattern, PureExpr, QueryExpr, ProcExpr, Expr<FX>, FX>;
pub type EffectExpr<FX> = sappho_ast_core::EffectExpr<FX, Expr<FX>>;
pub type FuncClause = sappho_ast_core::FuncDef<Pattern, PureExpr>;
pub type LetClause<FX> = sappho_ast_core::LetClause<Pattern, Expr<FX>>;
pub type LetExpr<FX> = sappho_ast_core::LetExpr<Pattern, Expr<FX>>;
pub type LookupExpr<FX> = sappho_ast_core::LookupExpr<Expr<FX>>;
pub type MatchClause<FX> = sappho_ast_core::MatchClause<Pattern, Expr<FX>>;
pub type MatchExpr<FX> = sappho_ast_core::MatchExpr<Pattern, Expr<FX>>;
pub type ObjectDef<FX> =
    sappho_ast_core::ObjectDef<Pattern, PureExpr, QueryExpr, ProcExpr, Expr<FX>>;
pub type ProcExpr = Expr<sappho_ast_core::ProcEffect>;
pub type ProcClause = sappho_ast_core::ProcDef<ProcExpr>;
pub type PureExpr = Expr<sappho_ast_core::PureEffect>;
pub type QueryClause = sappho_ast_core::QueryDef<QueryExpr>;
pub type QueryExpr = Expr<sappho_ast_core::QueryEffect>;
pub use self::expr::Expr;
pub use self::pattern::{Pattern, UnpackPattern};
pub use sappho_ast_core::{Identifier, Literal};
