use chumsky::Parser as _;
use derive_more::{From, Into};
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{Confined, FuncDef, Let, Wise};

/// Boxed-Spanned-Expression
#[derive(Debug, PartialEq, From, Into)]
#[from(Wise<FX>)]
pub struct BoxWise<FX>(Box<Wise<FX>>)
where
    FX: Effect;

impl_from_via_confined!(BoxWise<FX>);

macro_rules! from_via_wise {
    ( $t:ty ) => {
        impl<FX> From<$t> for BoxWise<FX>
        where
            FX: Effect,
        {
            fn from(v: $t) -> Self {
                BoxWise::from(Wise::from(v))
            }
        }
    };
}

from_via_wise!(Confined<FX>);
from_via_wise!(Let<FX>);
from_via_wise!(FuncDef);

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
