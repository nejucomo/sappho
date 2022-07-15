use crate::error::BareError;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{EffectExpr, ProcExpr};
use sappho_gast::ProcEffects;

pub(crate) fn proc_effect(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, EffectExpr<ProcEffects>, Error = BareError> + '_ {
    effect()
        .then(pexpr)
        .map(|(fx, x)| EffectExpr::new(fx, Box::new(x)))
}

fn effect() -> impl Parser<char, ProcEffects, Error = BareError> {
    use ProcEffects::*;

    just('!').to(Invoke).or(just('$').to(Inquire))
}
