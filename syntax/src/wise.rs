mod fromimpls;
mod parsing;

use derive_more::{From, Into};
use sappho_effect::Effect;
use sappho_with_source::WithSource;

use crate::Expr;

/// **Wi**th **S**ource **E**xpression
///
/// An [Expr] with attached source code for errors and other diagnostics.
///
/// This is the top-level expression recursion point, without `Box`. Expression recursions to [Wise] are either through [Box] or another container, like [ListForm](sappho_listform::ListForm).
#[derive(Debug, From, Into)]
pub struct Wise<FX>(WithSource<Expr<FX>>)
where
    FX: Effect;

impl<FX, T> From<T> for Wise<FX>
where
    FX: Effect,
    Expr<FX>: From<T>,
{
    fn from(value: T) -> Self {
        Wise::from(WithSource::from(Expr::from(value)))
    }
}

impl<FX> PartialEq for Wise<FX>
where
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.parsed == other.0.parsed
    }
}
