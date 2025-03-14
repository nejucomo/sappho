use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{Confined, Interactions};

impl ParsableWith<ParseParams<'_>> for Interactions<ProcEffect> {
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        ProcEffect::parser()
            .repeated()
            .then(Confined::parser_with(pep))
            .map(Interactions::from)
    }
}

impl<FX> Unparse for Interactions<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        for fx in &self.effects {
            s.write(fx);
        }
        s.write(&self.confined);
    }
}
