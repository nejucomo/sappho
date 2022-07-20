//! The Eval Abstract Syntax Tree (east) is a subset of [`sappho-ast`] for evaluation

mod expr;
mod pattern;

pub type ApplicationExpr<FX> = sappho_astcore::ApplicationExpr<Expr<FX>>;
pub type CoreExpr<FX> = sappho_astcore::CoreExpr<Pattern, PureExpr, QueryExpr, Expr<FX>, FX>;
pub type EffectExpr<FX> = sappho_astcore::EffectExpr<FX, Expr<FX>>;
pub type FuncClause = sappho_astcore::FuncDef<Pattern, PureExpr>;
pub type LetClause<FX> = sappho_astcore::LetClause<Pattern, Expr<FX>>;
pub type LetExpr<FX> = sappho_astcore::LetExpr<Pattern, Expr<FX>>;
pub type LookupExpr<FX> = sappho_astcore::LookupExpr<Expr<FX>>;
pub type MatchClause<FX> = sappho_astcore::MatchClause<Pattern, Expr<FX>>;
pub type MatchExpr<FX> = sappho_astcore::MatchExpr<Pattern, Expr<FX>>;
pub type ObjectDef<FX> = sappho_astcore::ObjectDef<Pattern, PureExpr, QueryExpr, Expr<FX>>;
pub type ProcExpr = Expr<sappho_astcore::ProcEffects>;
pub type PureExpr = Expr<sappho_astcore::PureEffects>;
pub type QueryClause = sappho_astcore::QueryDef<QueryExpr>;
pub type QueryExpr = Expr<sappho_astcore::QueryEffects>;
pub use self::expr::Expr;
pub use self::pattern::{Pattern, UnpackPattern};
pub use sappho_astcore::{Identifier, Literal};
