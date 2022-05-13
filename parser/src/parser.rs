use crate::delimited::delimited;
use crate::space::ws;
use crate::Error;
use chumsky::error::Simple;
use chumsky::primitive::just;
use chumsky::recursive::{recursive, Recursive};
use chumsky::text;
use chumsky::Parser;
use saplang_ast::{Expr, Literal, Pattern};
use std::str::FromStr;

pub(crate) fn expression() -> impl Parser<char, Expr, Error = Error> {
    recursive(expr).then_ignore(chumsky::primitive::end())
}

fn expr(expr: Recursive<'_, char, Expr, Error>) -> impl Parser<char, Expr, Error = Error> + '_ {
    let inner = parens_expr(expr.clone())
        .or(object_expr(expr.clone()))
        .or(let_expr(expr.clone()))
        .or(func_expr(expr.clone()))
        .or(reference())
        .or(literal())
        .or(list(expr));

    let innerws = inner.then_ignore(ws().or_not());

    innerws.repeated().at_least(1).map(|exprs| {
        exprs
            .into_iter()
            .reduce(Expr::application)
            .expect(".at_least(1) postcondition failed.")
    })
}

fn parens_expr(
    expr: Recursive<'_, char, Expr, Error>,
) -> impl Parser<char, Expr, Error = Error> + '_ {
    delimited('(', expr, ')')
}

fn literal() -> impl Parser<char, Expr, Error = Error> {
    use Literal::*;

    number().map(Num).map(Expr::Lit)
}

fn number() -> impl Parser<char, f64, Error = Error> {
    text::digits(10).try_map(|digs: String, span| {
        f64::from_str(&digs).map_err(|e| Simple::custom(span, e.to_string()))
    })
}

fn reference() -> impl Parser<char, Expr, Error = Error> {
    text::ident().map(Expr::Ref)
}

fn list(expr: Recursive<'_, char, Expr, Error>) -> impl Parser<char, Expr, Error = Error> + '_ {
    use crate::listform::list_form;

    list_form(expr).map(Expr::List)
}

fn let_expr(expr: Recursive<'_, char, Expr, Error>) -> impl Parser<char, Expr, Error = Error> + '_ {
    text::keyword("let")
        .then_ignore(ws())
        .ignore_then(pattern())
        .then_ignore(just('=').delimited_by(ws(), ws()))
        .then(expr.clone())
        .then_ignore(just(';'))
        .then_ignore(ws())
        .then(expr)
        .map(|((binding, bindexpr), tail)| Expr::let_expr(binding, bindexpr, tail))
}

fn func_expr(
    expr: Recursive<'_, char, Expr, Error>,
) -> impl Parser<char, Expr, Error = Error> + '_ {
    func_clause(expr).map(|(binding, body)| Expr::func_expr(binding, body))
}

fn func_clause(
    expr: Recursive<'_, char, Expr, Error>,
) -> impl Parser<char, (Pattern, Expr), Error = Error> + '_ {
    text::keyword("fn")
        .then_ignore(ws())
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(expr)
}

fn object_expr(
    expr: Recursive<'_, char, Expr, Error>,
) -> impl Parser<char, Expr, Error = Error> + '_ {
    delimited('{', func_clause(expr).or_not(), '}').map(Expr::object_expr)
}

fn pattern() -> impl Parser<char, Pattern, Error = Error> {
    text::ident()
}
