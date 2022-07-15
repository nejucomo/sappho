//! The Eval Abstract Syntax Tree (east) is a subset of [`sappho-ast`] for evaluation

mod expr;
mod pattern;

pub type ApplicationExpr<FX> = sappho_gast::ApplicationExpr<Expr<FX>>;
pub type EffectExpr<FX> = sappho_gast::EffectExpr<FX, Expr<FX>>;
pub type FuncClause = sappho_gast::FuncDef<Pattern, PureExpr>;
pub type LetClause<FX> = sappho_gast::LetClause<Pattern, Expr<FX>>;
pub type LetExpr<FX> = sappho_gast::LetExpr<Pattern, Expr<FX>>;
pub type LookupExpr<FX> = sappho_gast::LookupExpr<Expr<FX>>;
pub type MatchClause<FX> = sappho_gast::MatchClause<Pattern, Expr<FX>>;
pub type MatchExpr<FX> = sappho_gast::MatchExpr<Pattern, Expr<FX>>;
pub type ObjectDef<FX> = sappho_gast::ObjectDef<Pattern, PureExpr, QueryExpr, Expr<FX>>;
pub type ProcExpr = Expr<sappho_gast::ProcEffects>;
pub type PureExpr = Expr<sappho_gast::PureEffects>;
pub type QueryClause = sappho_gast::QueryDef<QueryExpr>;
pub type QueryExpr = Expr<sappho_gast::QueryEffects>;
pub use self::expr::Expr;
pub use self::pattern::{Pattern, UnpackPattern};
pub use sappho_gast::{Identifier, Literal};
