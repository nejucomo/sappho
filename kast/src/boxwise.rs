use chumsky::Parser as _;
use derive_more::{Deref, From, Into};
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_source::SourceCodeRef;
use sappho_unparse::{Stream, Unparse};
use sappho_with_source::WithSource;

use crate::{KastProvider, Wise};

/// Boxed-Spanned-Expression
#[derive(Clone, Debug, PartialEq, From, Into, Deref)]
#[from(Wise<K, FX>)]
#[deref(forward)]
pub struct BoxWise<K, FX>(Box<Wise<K, FX>>)
where
    K: KastProvider,
    FX: Effect;

impl<K, FX> BoxWise<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    pub fn new<T, C>(expr: T, sourcecode: C) -> Self
    where
        T: Into<K::Expr<FX>>,
        C: Into<Option<SourceCodeRef>>,
    {
        Wise::new(expr, sourcecode).into()
    }

    pub fn unwrap(self) -> Wise<K, FX> {
        *self.0
    }
}

impl<K, FX> From<WithSource<K::Expr<FX>>> for BoxWise<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: WithSource<K::Expr<FX>>) -> Self {
        Wise::from(value).into()
    }
}

impl<K, FX, T> ParsableWith<T> for BoxWise<K, FX>
where
    K: KastProvider,
    FX: Effect,
    Wise<K, FX>: ParsableWith<T>,
{
    fn make_parser_with(param: T) -> impl Parser<Self> {
        Wise::<K, FX>::parser_with(param).map(Box::new).map(Self)
    }
}

impl<K, FX> Unparse for BoxWise<K, FX>
where
    K: KastProvider,
    FX: Effect,
    Wise<K, FX>: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<K, FX> RestrictFrom<BoxWise<K, ProcEffect>> for BoxWise<K, FX>
where
    K: KastProvider,
    FX: Effect,
    Wise<K, FX>: RestrictFrom<Wise<K, ProcEffect>>,
{
    fn restrict(src: BoxWise<K, ProcEffect>) -> Result<BoxWise<K, FX>, Restriction> {
        Box::restrict(src.0).map(BoxWise)
    }
}

impl<K, FX> From<BoxWise<K, FX>> for (K::Expr<FX>, Option<SourceCodeRef>)
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: BoxWise<K, FX>) -> Self {
        (*value.0).into()
    }
}
