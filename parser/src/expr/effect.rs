use crate::error::BareError;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{ProcEffects, ProcExpr};

pub(crate) fn proc_effect(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcEffects, Error = BareError> + '_ {
    use ProcEffects::*;

    effect(pexpr.clone(), '!', Evoke).or(effect(pexpr, '$', Inquire))
}

fn effect<'a, F: 'a>(
    pexpr: Recursive<'a, char, ProcExpr, BareError>,
    sym: char,
    f: F,
) -> impl Parser<char, ProcEffects, Error = BareError> + 'a
where
    F: Fn(Box<ProcExpr>) -> ProcEffects,
{
    just(sym).ignore_then(pexpr).map(Box::new).map(f)
}
