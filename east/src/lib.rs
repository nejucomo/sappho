mod expr;
mod provider;

use sappho_effect::{ProcEffect, PureEffect, QueryEffect};

pub use crate::expr::Expr;
pub use crate::provider::EastProvider;

// Aliases:
pub type PureExpr = BoxWise<PureEffect>;
pub type QueryExpr = BoxWise<QueryEffect>;
pub type ProcExpr = BoxWise<ProcEffect>;

pub type FuncDef = sappho_kast::FuncDef<EastProvider>;
pub type QueryDef = sappho_kast::QueryDef<EastProvider>;
pub type ProcDef = sappho_kast::ProcDef<EastProvider>;

pub type BoxWise<FX> = sappho_kast::BoxWise<EastProvider, FX>;
pub type Let<FX> = sappho_kast::Let<EastProvider, FX>;
pub type Match<FX> = sappho_kast::Match<EastProvider, FX>;
pub type ObjectDef<FX> = sappho_kast::ObjectDef<EastProvider, FX>;
pub type Wise<FX> = sappho_kast::Wise<EastProvider, FX>;
