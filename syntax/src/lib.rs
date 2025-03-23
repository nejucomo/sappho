/// # Todo
///
/// Move `ListForm` into this crate.
pub mod leftassoc;

mod applications;
mod boxwise;
mod confined;
mod expr;
mod funcdef;
mod interactions;
mod letexpr;
mod lookups;
mod matchexpr;
mod parens;
mod parseparams;
mod parserext;
mod pattern;
mod procdef;
mod querydef;
mod wise;

use sappho_ast_effect::{ProcEffect, PureEffect, QueryEffect};

pub use crate::applications::{Application, Applications};
pub use crate::boxwise::BoxWise;
pub use crate::confined::Confined;
pub use crate::expr::Expr;
pub use crate::funcdef::FuncDef;
pub use crate::interactions::Interactions;
pub use crate::letexpr::{Let, LetClause};
pub use crate::lookups::{Lookup, Lookups};
pub use crate::matchexpr::{Match, MatchClause};
pub use crate::parens::ParensExpr;
pub use crate::pattern::{BindPattern, Pattern};
pub use crate::procdef::ProcDef;
pub use crate::querydef::QueryDef;
pub use crate::wise::Wise;

// Top-level expressions for each effect kind:
pub type PureExpr = BoxWise<PureEffect>;
pub type QueryExpr = BoxWise<QueryEffect>;
pub type ProcExpr = BoxWise<ProcEffect>;

#[cfg(test)]
mod tests;
