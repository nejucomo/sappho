use crate::delimited::delimited;
use crate::parser::recursive::recursive_expr;
use crate::parser::universal::universal_expr;
use crate::space::ws;
use crate::Error;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::GenExpr;

pub(crate) fn base_expr<'a, FX: 'a, F, P>(
    nonapp: F,
    expr: Recursive<'a, char, GenExpr<FX>, Error>,
) -> impl Parser<char, GenExpr<FX>, Error = Error> + 'a
where
    F: FnOnce(Recursive<'a, char, GenExpr<FX>, Error>) -> P,
    P: Parser<char, GenExpr<FX>, Error = Error> + 'a,
{
    nonapp(expr)
        .then_ignore(ws().or_not())
        .repeated()
        .at_least(1)
        .map(|exprs| {
            exprs
                .into_iter()
                .reduce(GenExpr::application)
                .expect(".at_least(1) postcondition failed.")
        })
}

pub(crate) fn uncommon_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, Error>,
) -> impl Parser<char, GenExpr<FX>, Error = Error> + 'a {
    parens_expr(expr.clone())
        .or(universal_expr().map(GenExpr::Universal))
        .or(recursive_expr(expr).map(GenExpr::Recursive))
}

fn parens_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, Error>,
) -> impl Parser<char, GenExpr<FX>, Error = Error> + 'a {
    delimited('(', expr, ')')
}
