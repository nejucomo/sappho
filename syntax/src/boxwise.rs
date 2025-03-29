use chumsky::Parser as _;
use derive_more::{From, Into};
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::Wise;

/// Boxed-Spanned-Expression
#[derive(Debug, PartialEq, From, Into)]
pub struct BoxWise<FX>(Box<Wise<FX>>)
where
    FX: Effect;

impl<T, FX> From<T> for BoxWise<FX>
where
    FX: Effect,
    Wise<FX>: From<T>,
{
    fn from(t: T) -> Self {
        Self::from(Box::new(Wise::from(t)))
    }
}

impl<T, FX> ParsableWith<T> for BoxWise<FX>
where
    FX: Effect,
    Wise<FX>: ParsableWith<T>,
{
    fn make_parser_with(param: T) -> impl Parser<Self> {
        Wise::<FX>::parser_with(param).map(Box::new).map(Self)
    }
}

impl<FX> Unparse for BoxWise<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<FX> RestrictFrom<BoxWise<ProcEffect>> for BoxWise<FX>
where
    FX: Effect,
{
    fn restrict(src: BoxWise<ProcEffect>) -> Result<BoxWise<FX>, Restriction> {
        Box::restrict(src.0).map(BoxWise)
    }
}
