use crate::keyword::Keyword;
use crate::listform::list_form;
use crate::parser::pattern::pattern;
use crate::space::ws;
use crate::error::BareError;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::{GenExpr, LetExpr, RecursiveExpr};

pub(crate) fn recursive_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, BareError>,
) -> impl Parser<char, RecursiveExpr<FX>, Error = BareError> + 'a {
    use RecursiveExpr::*;

    list_form(expr.clone())
        .map(List)
        .or(let_expr(expr).map(Let))
}

fn let_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, BareError>,
) -> impl Parser<char, LetExpr<FX>, Error = BareError> + 'a {
    Keyword::Let
        .parser()
        .ignore_then(pattern())
        .then_ignore(just('=').delimited_by(ws(), ws()))
        .then(expr.clone())
        .then_ignore(just(';'))
        .then_ignore(ws())
        .then(expr)
        .map(|((binding, bindexpr), tail)| LetExpr {
            binding,
            bindexpr: Box::new(bindexpr),
            tail: Box::new(tail),
        })
}
