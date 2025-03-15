use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::restrict::RestrictInto;
use crate::{Confined, Interactions};

impl<FX> ParsableWith<ParseParams<'_>> for Interactions<FX>
where
    FX: Effect,
    ProcEffect: RestrictInto<FX>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
        FX::parser()
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
