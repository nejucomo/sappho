//! The Eval Abstract Syntax Tree (east) is a subset of [`sappho-ast`] for evaluation

mod effects;
mod expr;
mod pattern;

pub use sappho_gast::{Identifier, Literal};
pub type ApplicationExpr<FX> = sappho_gast::ApplicationExpr<Expr<FX>>;
pub type LetExpr<FX> = sappho_gast::LetExpr<Pattern, Expr<FX>>;
pub type LetClause<FX> = sappho_gast::LetClause<Pattern, Expr<FX>>;
pub type LookupExpr<FX> = sappho_gast::LookupExpr<Expr<FX>>;
pub type MatchExpr<FX> = sappho_gast::MatchExpr<Pattern, Expr<FX>>;
pub type MatchClause<FX> = sappho_gast::MatchClause<Pattern, Expr<FX>>;
pub type QueryClause = sappho_gast::QueryDef<QueryExpr>;
pub type FuncClause = sappho_gast::FuncDef<Pattern, PureExpr>;
pub type ObjectDef<FX> = sappho_gast::ObjectDef<Pattern, PureExpr, QueryExpr, Expr<FX>>;

pub use self::effects::{
    AstFxFor, FromFx, ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr,
};
pub use self::expr::Expr;
pub use self::pattern::{Pattern, UnpackPattern};
