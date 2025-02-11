use crate::error::BareError;
use crate::expr::pattern::pattern;
use crate::keyword::Keyword;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{
    Expr, LetClause, LetExpr, ListExpr, MatchClause, MatchExpr, ProcEffect, ProcExpr,
};

pub(crate) fn recursive_expr(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    use Expr::List;

    list_expr(expr.clone())
        .map(List)
        .or(let_expr(expr.clone()).map(Expr::from))
        .or(match_expr(expr).map(Expr::from))
}

fn list_expr(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<char, ListExpr<ProcEffect>, Error = BareError> + '_ {
    use crate::listform::list_form;

    list_form(expr.clone(), expr.map(Box::new)).labelled("list-expression")
}

fn let_expr(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<char, LetExpr<ProcEffect>, Error = BareError> + '_ {
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

fn let_clause(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<char, LetClause<ProcEffect>, Error = BareError> + '_ {
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

fn match_expr(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<char, MatchExpr<ProcEffect>, Error = BareError> + '_ {
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

fn match_clause(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<char, MatchClause<ProcEffect>, Error = BareError> + '_ {
    pattern()
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(expr)
        .map(|(pattern, body)| MatchClause {
            pattern,
            body: Box::new(body),
        })
        .labelled("match-clause")
}
