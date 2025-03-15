use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{BoxWise, Wise};

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
