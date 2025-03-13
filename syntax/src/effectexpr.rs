use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::{Confined, EffectExpr, ProcExprParser};

impl ParsableWith<ProcExprParser<'_>> for EffectExpr<ProcEffect> {
    fn make_parser_with(pep: ProcExprParser<'_>) -> impl Parser<Self> {
        ProcEffect::parser()
            .repeated()
            .then(Confined::parser_with(pep))
            .map(EffectExpr::from)
    }
}

impl<FX> Unparse for EffectExpr<FX>
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
