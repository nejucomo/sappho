use crate::space::ws;
use crate::Error;
use chumsky::error::Simple;
use chumsky::primitive::just;
use chumsky::recursive::{recursive, Recursive};
use chumsky::text;
use chumsky::Parser;
use saplang_ast::{Application, Expr, FuncExpr, Identifier, LetExpr, Literal, Pattern};
use std::str::FromStr;

pub fn expression() -> impl Parser<char, Expr, Error = Error> {
    recursive(expr).then_ignore(chumsky::primitive::end())
}

fn expr(expr: Recursive<'_, char, Expr, Error>) -> impl Parser<char, Expr, Error = Error> + '_ {
    use Expr::*;

    let inner = parens_expr(expr.clone())
        .or(let_expr(expr.clone()).map(|le| Let(Box::new(le))))
        .or(func_expr(expr.clone()).map(|fe| Func(Box::new(fe))))
        .or(reference().map(Ref))
        .or(literal().map(Lit))
        .or(list(expr).map(List));

    let innerws = inner.then_ignore(ws().or_not());

    innerws.repeated().at_least(1).map(|exprs| {
        exprs
            .into_iter()
            .reduce(|target, argument| Expr::Apply(Box::new(Application { target, argument })))
            .expect(".at_least(1) postcondition failed.")
    })
}

fn parens_expr(
    expr: Recursive<'_, char, Expr, Error>,
) -> impl Parser<char, Expr, Error = Error> + '_ {
    expr.delimited_by(just('(').then_ignore(ws().or_not()), just(')'))
}

fn literal() -> impl Parser<char, Literal, Error = Error> {
    use Literal::*;

    number().map(Num)
}

fn number() -> impl Parser<char, f64, Error = Error> {
    text::digits(10).try_map(|digs: String, span| {
        f64::from_str(&digs).map_err(|e| Simple::custom(span, e.to_string()))
    })
}

fn reference() -> impl Parser<char, Identifier, Error = Error> {
    text::ident()
}

fn list(
    expr: Recursive<'_, char, Expr, Error>,
) -> impl Parser<char, Vec<Expr>, Error = Error> + '_ {
    use crate::listform::list_form;

    list_form(expr)
}

fn let_expr(
    expr: Recursive<'_, char, Expr, Error>,
) -> impl Parser<char, LetExpr, Error = Error> + '_ {
    text::keyword("let")
        .then_ignore(ws())
        .ignore_then(pattern())
        .then_ignore(just('=').delimited_by(ws(), ws()))
        .then(expr.clone())
        .then_ignore(just(';'))
        .then_ignore(ws())
        .then(expr)
        .map(|((binding, bindexpr), tail)| LetExpr {
            binding,
            bindexpr,
            tail,
        })
}

fn func_expr(
    expr: Recursive<'_, char, Expr, Error>,
) -> impl Parser<char, FuncExpr, Error = Error> + '_ {
    text::keyword("fn")
        .then_ignore(ws())
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(expr)
        .map(|(binding, body)| FuncExpr { binding, body })
}

fn pattern() -> impl Parser<char, Pattern, Error = Error> {
    text::ident()
}
