//! The Eval Abstract Syntax Tree (east) is a subset of [`sappho-ast`] for evaluation

mod effects;
mod expr;

pub use sappho_gast::{Identifier, ListExpr, Literal, Pattern, UnpackPattern};
pub type ApplicationExpr<FX> = sappho_gast::ApplicationExpr<GenExpr<FX>>;
pub type LetExpr<FX> = sappho_gast::LetExpr<GenExpr<FX>>;
pub type LetClause<FX> = sappho_gast::LetClause<GenExpr<FX>>;
pub type LookupExpr<FX> = sappho_gast::LookupExpr<GenExpr<FX>>;
pub type MatchExpr<FX> = sappho_gast::MatchExpr<GenExpr<FX>>;
pub type MatchClause<FX> = sappho_gast::MatchClause<GenExpr<FX>>;
pub type QueryClause = sappho_gast::QueryDef<QueryExpr>;
pub type FuncClause = sappho_gast::FuncDef<PureExpr>;
pub type ObjectDef = sappho_gast::ObjectDef<PureExpr, QueryExpr>;

pub use self::effects::{
    AstFxFor, FromFx, ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr,
};
pub use self::expr::GenExpr;
