mod fromimpls;
mod parsing;

use derive_more::{Deref, From};
use sappho_effect::Effect;
use sappho_source::SourceCodeRef;
use sappho_with_source::WithSource;

use crate::KastProvider;

/// **Wi**th **S**ource **E**xpression
///
/// The top-level expression entry-point with attached source code for errors and other diagnostics. This contains the [KastProvider] expression directly; see [BoxWise](crate::BoxWise) for heap-stored [Wise] expressions.
#[derive(Clone, Debug, PartialEq, From, Deref)]
pub struct Wise<K, FX>(WithSource<K::Expr<FX>>)
where
    K: KastProvider,
    FX: Effect;

impl<K, FX> Wise<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    pub fn new<T, C>(expr: T, sourcecode: C) -> Self
    where
        T: Into<K::Expr<FX>>,
        C: Into<Option<SourceCodeRef>>,
    {
        Wise(WithSource::new(expr, sourcecode))
    }

    pub fn unwrap(self) -> WithSource<K::Expr<FX>> {
        self.0
    }
}
