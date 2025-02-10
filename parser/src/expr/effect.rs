use crate::error::BareError;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{EffectExpr, ProcExpr};
use sappho_ast_core::ProcEffect;

pub(crate) fn proc_effect(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, EffectExpr<ProcEffect>, Error = BareError> + '_ {
    effect()
        .then(pexpr)
        .map(|(fx, x)| EffectExpr::new(fx, Box::new(x)))
}

fn effect() -> impl Parser<char, ProcEffect, Error = BareError> {
    use ProcEffect::*;

    just('!').to(Invoke).or(just('$').to(Inquire))
}
