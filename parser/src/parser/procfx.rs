use crate::space::ws;
use crate::Error;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::{ProcEffects, ProcExpr};

pub(crate) fn proc_effect(
    pexpr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ProcEffects, Error = Error> + '_ {
    inquire(pexpr.clone()).or(evoke(pexpr))
}

fn inquire(
    pexpr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ProcEffects, Error = Error> + '_ {
    just('$')
        .then_ignore(ws())
        .ignore_then(pexpr)
        .map(Box::new)
        .map(ProcEffects::Inquire)
}

fn evoke(
    pexpr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ProcEffects, Error = Error> + '_ {
    just('!')
        .then_ignore(ws())
        .ignore_then(pexpr)
        .map(Box::new)
        .map(ProcEffects::Evoke)
}
