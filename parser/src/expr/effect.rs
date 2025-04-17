use crate::error::BareError;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser as _;
use sappho_ast::{Ast, ProcExpr};
use sappho_ast_core::EffectExpr;
use sappho_ast_effect::ProcEffect;
use sappho_parsable::Parser;

pub(crate) fn proc_effect(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<EffectExpr<Ast, ProcEffect>> + '_ {
    effect()
        .then(pexpr)
        .map(|(fx, x)| EffectExpr::new(fx, Box::new(x)))
}

fn effect() -> impl Parser<ProcEffect> {
    use ProcEffect::*;

    just('!').to(Invoke).or(just('$').to(Inquire))
}
