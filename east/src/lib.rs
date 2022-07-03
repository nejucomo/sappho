//! The Eval Abstract Syntax Tree (east) is a subset of [`sappho-ast`] for evaluation

mod effects;
mod expr;
mod pattern;

pub use sappho_gast::{Identifier, ListExpr, Literal};
pub type ApplicationExpr<FX> = sappho_gast::ApplicationExpr<GenExpr<FX>>;
pub type LetExpr<FX> = sappho_gast::LetExpr<Pattern, GenExpr<FX>>;
pub type LetClause<FX> = sappho_gast::LetClause<Pattern, GenExpr<FX>>;
pub type LookupExpr<FX> = sappho_gast::LookupExpr<GenExpr<FX>>;
pub type MatchExpr<FX> = sappho_gast::MatchExpr<Pattern, GenExpr<FX>>;
pub type MatchClause<FX> = sappho_gast::MatchClause<Pattern, GenExpr<FX>>;
pub type QueryClause = sappho_gast::QueryDef<QueryExpr>;
pub type FuncClause = sappho_gast::FuncDef<Pattern, PureExpr>;
pub type ObjectDef = sappho_gast::ObjectDef<Pattern, PureExpr, QueryExpr>;

pub use self::effects::{
    AstFxFor, FromFx, ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr,
};
pub use self::expr::GenExpr;
pub use self::pattern::{Pattern, UnpackPattern};
