use crate::error::BareError;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast_effect::ProcEffect;
use sappho_ast_kernel::Interaction;
use sappho_ast_rich::{Ast, ProcExpr};

pub(crate) fn proc_effect(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, Interaction<Ast, ProcEffect>, Error = BareError> + '_ {
    effect()
        .then(pexpr)
        .map(|(fx, x)| Interaction::new(fx, Box::new(x)))
}

fn effect() -> impl Parser<char, ProcEffect, Error = BareError> {
    use ProcEffect::*;

    just('!').to(Invoke).or(just('$').to(Inquire))
}
