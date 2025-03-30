mod fromimpls;
mod parsing;

use derive_more::From;
use sappho_effect::Effect;
use sappho_with_source::WithSource;

use crate::KastProvider;

/// **Wi**th **S**ource **E**xpression
///
/// An [Expr] with attached source code for errors and other diagnostics.
///
/// This is the top-level expression recursion point, without `Box`. Expression recursions to [Wise] are either through [Box] or another container, like [ListForm](sappho_listform::ListForm).
#[derive(Clone, Debug, PartialEq, From)]
pub struct Wise<K, FX>(WithSource<K::Expr<FX>>)
where
    K: KastProvider,
    FX: Effect;
