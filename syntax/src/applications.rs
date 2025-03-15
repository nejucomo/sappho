use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::primitive::space;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::leftassoc::LeftAssoc;
use crate::parseparams::ParseParams;
use crate::restrict::RestrictInto;
use crate::{Application, Applications, Lookups};

impl<FX> ParsableWith<ParseParams<'_>> for Applications<FX>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn make_parser_with(pp: ParseParams<'_>) -> impl Parser<Self> {
        LeftAssoc::parser_with(pp).map(Self)
    }
}

impl<FX> ParsableWith<ParseParams<'_>> for Application<FX>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
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
