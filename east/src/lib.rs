mod expr;
mod provider;

use sappho_effect::{ProcEffect, PureEffect, QueryEffect};
use sappho_listform::ListForm;

pub use crate::expr::Expr;
pub use crate::provider::EastProvider;

pub type PureExpr = BoxWise<PureEffect>;
pub type QueryExpr = BoxWise<QueryEffect>;
pub type ProcExpr = BoxWise<ProcEffect>;

pub type BoxWise<FX> = sappho_kast::BoxWise<EastProvider, FX>;
pub type Let<FX> = sappho_kast::Let<EastProvider, FX>;
pub type Match<FX> = sappho_kast::Match<EastProvider, FX>;
pub type ObjectDef<FX> = sappho_kast::ObjectDef<EastProvider, FX>;
pub type Wise<FX> = sappho_kast::Wise<EastProvider, FX>;

pub type ListDef<FX> = ListForm<Wise<FX>, BoxWise<FX>>;
