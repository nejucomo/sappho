use crate::error::BareError;
use crate::expr::ProcRecursion;
use chumsky::primitive::just;
use chumsky::Parser;
use sappho_ast::Ast;
use sappho_ast_core::{BoxExpr, EffectExpr, ProcEffect};

pub(crate) fn proc_effect(
    pexpr: ProcRecursion<'_>,
) -> impl Parser<char, EffectExpr<Ast, ProcEffect>, Error = BareError> + '_ {
    effect()
        .then(pexpr)
        .map(|(fx, x)| EffectExpr::new(fx, BoxExpr::from(x)))
}

fn effect() -> impl Parser<char, ProcEffect, Error = BareError> {
    use ProcEffect::*;

    just('!').to(Invoke).or(just('$').to(Inquire))
}
