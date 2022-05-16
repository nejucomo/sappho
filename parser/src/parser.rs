mod base;
mod common;
mod pattern;
mod recursive;
mod universal;

use crate::Error;
use chumsky::Parser;
use saplang_ast::{PureExpr, QueryExpr};

pub(crate) fn expression() -> impl Parser<char, PureExpr, Error = Error> {
    use chumsky::primitive::end;

    pure_expr().then_ignore(end())
}

fn pure_expr() -> impl Parser<char, PureExpr, Error = Error> {
    chumsky::recursive::recursive(|expr| {
        use self::base::{base_expr, uncommon_expr};
        use self::common::purectx;

        base_expr(
            |expr| uncommon_expr(expr.clone()).or(purectx::common_expr(expr).map(PureExpr::Common)),
            expr,
        )
    })
}

fn query_expr() -> impl Parser<char, QueryExpr, Error = Error> {
    chumsky::recursive::recursive(|expr| {
        use self::base::{base_expr, uncommon_expr};
        use self::common::queryctx;

        base_expr(
            |expr| {
                uncommon_expr(expr.clone()).or(queryctx::common_expr(expr).map(QueryExpr::Common))
            },
            expr,
        )
    })
}
