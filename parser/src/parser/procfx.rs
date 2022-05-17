use crate::space::ws;
use crate::Error;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::{ProcEffects, ProcExpr};

pub(crate) fn proc_effect(
    pexpr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ProcEffects, Error = Error> + '_ {
    use ProcEffects::*;

    effect(pexpr.clone(), '!', Evoke).or(effect(pexpr, '$', Inquire))
}

fn effect<'a, F: 'a>(
    pexpr: Recursive<'a, char, ProcExpr, Error>,
    sym: char,
    f: F,
) -> impl Parser<char, ProcEffects, Error = Error> + 'a
where
    F: Fn(Box<ProcExpr>) -> ProcEffects,
{
    just(sym)
        .then_ignore(ws().or_not())
        .ignore_then(pexpr)
        .map(Box::new)
        .map(f)
}
