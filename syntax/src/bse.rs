use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::spanned::Spanned;
use crate::{SEParser, BSE, SE};

impl Parsable for BSE<ProcEffect> {
    fn parser() -> impl Parser<Self> {
        SE::parser().map(Self::from)
    }
}

impl ParsableWith<SEParser<'_>> for BSE<ProcEffect> {
    fn make_parser_with(sep: SEParser<'_>) -> impl Parser<Self> {
        sep.map(Self::from)
    }
}

impl Parsable for SE<ProcEffect> {
    fn parser() -> impl Parser<Self> {
        chumsky::recursive::recursive(Self::parser_with)
    }
}

impl ParsableWith<SEParser<'_>> for SE<ProcEffect> {
    fn make_parser_with(sep: SEParser<'_>) -> impl Parser<Self> {
        Spanned::parser_with(sep).map(Self)
    }
}

impl<FX> Unparse for BSE<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<FX> Unparse for SE<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
