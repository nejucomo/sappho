use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::primitive::space;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::leftassoc::LeftAssoc;
use crate::{Application, Applications, Lookups, SEParser};

impl ParsableWith<SEParser<'_>> for Applications<ProcEffect> {
    fn make_parser_with(pep: SEParser<'_>) -> impl Parser<Self> {
        LeftAssoc::parser_with(pep).map(Self)
    }
}

impl ParsableWith<SEParser<'_>> for Application<ProcEffect> {
    fn make_parser_with(pep: SEParser<'_>) -> impl Parser<Self> {
        space().ignore_then(Lookups::parser_with(pep)).map(Self)
    }
}

impl<FX> Unparse for Applications<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}

impl<FX> Unparse for Application<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(" ");
        self.0.unparse_into(s);
    }
}
