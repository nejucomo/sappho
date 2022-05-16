mod base;
mod common;
mod pattern;
mod recursive;
mod universal;

use crate::Error;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::PureExpr;

pub(crate) fn expression() -> impl Parser<char, PureExpr, Error = Error> {
    use chumsky::primitive::end;

    pure_expr().then_ignore(end())
}

fn pure_expr() -> impl Parser<char, PureExpr, Error = Error> {
    chumsky::recursive::recursive(pure_expr_rec)
}

fn pure_expr_rec(
    expr: Recursive<'_, char, PureExpr, Error>,
) -> impl Parser<char, PureExpr, Error = Error> + '_ {
    use self::base::{base_expr, uncommon_expr};
    use self::common::common_expr_within_pure;

    base_expr(
        |expr| uncommon_expr(expr.clone()).or(common_expr_within_pure(expr).map(PureExpr::Common)),
        expr,
    )
}
