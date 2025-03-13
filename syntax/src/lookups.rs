use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_identifier::RcId;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::leftassoc::LeftAssoc;
use crate::{Lookup, Lookups, ProcExprParser};

impl ParsableWith<ProcExprParser<'_>> for Lookups<ProcEffect> {
    fn make_parser_with(pep: ProcExprParser<'_>) -> impl Parser<Self> {
        LeftAssoc::parser_with(pep).map(Self)
    }
}

impl ParsableWith<ProcExprParser<'_>> for Lookup {
    fn make_parser_with(_: ProcExprParser<'_>) -> impl Parser<Self> {
        just('.').ignore_then(RcId::parser()).map(Self)
    }
}

impl<FX> Unparse for Lookups<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s);
    }
}

impl Unparse for Lookup {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(".");
        self.0.unparse_into(s);
    }
}
