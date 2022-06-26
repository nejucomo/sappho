use crate::error::BareError;
use crate::expr::pattern::pattern;
use crate::keyword::Keyword;
use crate::listform::list_form;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{GenExpr, LetClause, LetExpr, ListForm, MatchClause, MatchExpr};

pub(crate) fn recursive_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, BareError>,
) -> impl Parser<char, GenExpr<FX>, Error = BareError> + 'a {
    use GenExpr::*;

    list_expr(expr.clone())
        .map(List)
        .or(let_expr(expr.clone()).map(Let))
        .or(match_expr(expr).map(Match))
}

fn list_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, BareError>,
) -> impl Parser<char, ListForm<GenExpr<FX>>, Error = BareError> + 'a {
    list_form(expr).labelled("list-expression")
}

fn let_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, BareError>,
) -> impl Parser<char, LetExpr<FX>, Error = BareError> + 'a {
    let_clause(expr.clone())
        .then_ignore(ws())
        .repeated()
        .at_least(1)
        .then(expr)
        .map(|(clauses, tail)| LetExpr {
            clauses,
            tail: Box::new(tail),
        })
        .labelled("let-expression")
}

fn let_clause<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, BareError>,
) -> impl Parser<char, LetClause<FX>, Error = BareError> + 'a {
    Keyword::Let
        .parser()
        .ignore_then(pattern())
        .then_ignore(just('=').delimited_by(ws(), ws()))
        .then(expr.clone())
        .then_ignore(just(';'))
        .map(|(binding, bindexpr)| LetClause {
            binding,
            bindexpr: Box::new(bindexpr),
        })
}

fn match_expr<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, BareError>,
) -> impl Parser<char, MatchExpr<FX>, Error = BareError> + 'a {
    use crate::delimited::delimited;

    Keyword::Match
        .parser()
        .ignore_then(expr.clone())
        .then_ignore(ws())
        .then(delimited(
            '{',
            match_clause(expr)
                .separated_by(just(',').then(ws()))
                .allow_trailing(),
            '}',
        ))
        .map(|(target, clauses)| MatchExpr {
            target: Box::new(target),
            clauses,
        })
        .labelled("match-expression")
}

fn match_clause<'a, FX: 'a>(
    expr: Recursive<'a, char, GenExpr<FX>, BareError>,
) -> impl Parser<char, MatchClause<FX>, Error = BareError> + 'a {
    pattern()
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(expr)
        .map(|(pattern, body)| MatchClause {
            pattern,
            body: Box::new(body),
        })
        .labelled("match-clause")
}
