use crate::error::BareError;
use crate::RecExpr;
use chumsky::primitive::just;
use chumsky::Parser;
use sappho_ast::Ast;
use sappho_ast_core::EffectExpr;
use sappho_ast_effect::ProcEffect;

pub(crate) fn proc_effect(
    rec: RecExpr<'_>,
) -> impl Parser<char, EffectExpr<Ast, ProcEffect>, Error = BareError> + '_ {
    effect().then(rec).map(|(fx, x)| EffectExpr::new(fx, x))
}

fn effect() -> impl Parser<char, ProcEffect, Error = BareError> {
    use ProcEffect::*;

    just('!').to(Invoke).or(just('$').to(Inquire))
}
