mod application;
mod expr;
mod interaction;
mod lookup;
mod provider;
mod transform;

use sappho_effect::{Effect, ProcEffect, PureEffect, QueryEffect};
use sappho_listform::ListForm;
use sappho_syntax as syntax;

use crate::transform::TransformInto;

pub fn from_syntax<FX>(w: syntax::Wise<FX>) -> Wise<FX>
where
    FX: Effect,
{
    w.transform_into()
}

pub use crate::application::Application;
pub use crate::expr::Expr;
pub use crate::interaction::Interaction;
pub use crate::lookup::Lookup;
pub use crate::provider::EastProvider;

pub type PureExpr = BoxWise<PureEffect>;
pub type QueryExpr = BoxWise<QueryEffect>;
pub type ProcExpr = BoxWise<ProcEffect>;

pub type FuncDef = sappho_kast::FuncDef<EastProvider>;
pub type QueryDef = sappho_kast::QueryDef<EastProvider>;
pub type ProcDef = sappho_kast::ProcDef<EastProvider>;

pub type BoxWise<FX> = sappho_kast::BoxWise<EastProvider, FX>;
pub type LetClause<FX> = sappho_kast::LetClause<EastProvider, FX>;
pub type Let<FX> = sappho_kast::Let<EastProvider, FX>;
pub type MatchClause<FX> = sappho_kast::MatchClause<EastProvider, FX>;
pub type Match<FX> = sappho_kast::Match<EastProvider, FX>;
pub type ObjectDef<FX> = sappho_kast::ObjectDef<EastProvider, FX>;
pub type Wise<FX> = sappho_kast::Wise<EastProvider, FX>;

pub type ListDef<FX> = ListForm<Wise<FX>, BoxWise<FX>>;
