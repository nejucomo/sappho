/// # Todo
///
/// Move `ListForm` into this crate.
pub mod leftassoc;

mod applications;
mod confined;
mod expr;
mod interactions;
mod lookups;
mod parens;
mod provider;

use sappho_effect::{ProcEffect, PureEffect, QueryEffect};

pub use crate::applications::{Application, Applications};
pub use crate::confined::Confined;
pub use crate::expr::Expr;
pub use crate::interactions::Interactions;
pub use crate::lookups::{Lookup, Lookups};
pub use crate::parens::ParensExpr;
pub use crate::provider::SyntaxProvider;

// Aliases:
pub type PureExpr = BoxWise<PureEffect>;
pub type QueryExpr = BoxWise<QueryEffect>;
pub type ProcExpr = BoxWise<ProcEffect>;

pub type FuncDef = sappho_kast::FuncDef<SyntaxProvider>;
pub type QueryDef = sappho_kast::QueryDef<SyntaxProvider>;
pub type ProcDef = sappho_kast::ProcDef<SyntaxProvider>;

pub type BoxWise<FX> = sappho_kast::BoxWise<SyntaxProvider, FX>;
pub type Let<FX> = sappho_kast::Let<SyntaxProvider, FX>;
pub type LetClause<FX> = sappho_kast::LetClause<SyntaxProvider, FX>;
pub type Match<FX> = sappho_kast::Match<SyntaxProvider, FX>;
pub type MatchClause<FX> = sappho_kast::MatchClause<SyntaxProvider, FX>;
pub type ObjectDef<FX> = sappho_kast::ObjectDef<SyntaxProvider, FX>;
pub type Wise<FX> = sappho_kast::Wise<SyntaxProvider, FX>;

#[cfg(test)]
mod tests;
