use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_identifier::RcId;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::proc::ProcExprParser;
use crate::{EffectExpr, Lookup, Lookups};

impl ParsableWith<ProcExprParser<'_>> for Lookups<ProcEffect> {
    fn make_parser_with(pep: ProcExprParser<'_>) -> impl Parser<Self> {
        EffectExpr::parser_with(pep)
            .left_assoc_then_rights(Lookup::parser())
            .map(Self)
    }
}

impl Parsable for Lookup {
    fn parser() -> impl Parser<Self> {
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
