use chumsky::Parser as _;
use derive_more::{From, Into};
use sappho_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{KastProvider, Wise};

/// Boxed-Spanned-Expression
#[derive(Clone, Debug, PartialEq, From, Into)]
pub struct BoxWise<K, FX>(Box<Wise<K, FX>>)
where
    K: KastProvider,
    FX: Effect;

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
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<K, FX> RestrictFrom<BoxWise<K, ProcEffect>> for BoxWise<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn restrict(src: BoxWise<K, ProcEffect>) -> Result<BoxWise<K, FX>, Restriction> {
        Box::restrict(src.0).map(BoxWise)
    }
}
