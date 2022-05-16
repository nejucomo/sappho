use crate::delimited::delimited;
use crate::parser::common::common_expr;
use crate::parser::recursive::recursive_expr;
use crate::parser::universal::universal_expr;
use crate::space::ws;
use crate::Error;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::PureExpr;

pub(super) fn pure_expr() -> impl Parser<char, PureExpr, Error = Error> {
    chumsky::recursive::recursive(pure_expr_rec)
}

fn pure_expr_rec(
    expr: Recursive<'_, char, PureExpr, Error>,
) -> impl Parser<char, PureExpr, Error = Error> + '_ {
    non_app_expr(expr)
        .then_ignore(ws().or_not())
        .repeated()
        .at_least(1)
        .map(|exprs| {
            exprs
                .into_iter()
                .reduce(PureExpr::application)
                .expect(".at_least(1) postcondition failed.")
        })
}

fn non_app_expr(
    expr: Recursive<'_, char, PureExpr, Error>,
) -> impl Parser<char, PureExpr, Error = Error> + '_ {
    parens_expr(expr.clone())
        .or(universal_expr().map(PureExpr::Universal))
        .or(common_expr(expr.clone()).map(PureExpr::Common))
        .or(recursive_expr(expr).map(PureExpr::Recursive))
}

fn parens_expr(
    expr: Recursive<'_, char, PureExpr, Error>,
) -> impl Parser<char, PureExpr, Error = Error> + '_ {
    delimited('(', expr, ')')
}
