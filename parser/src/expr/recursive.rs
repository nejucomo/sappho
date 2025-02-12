use crate::error::BareError;
use crate::expr::pattern::pattern;
use crate::expr::ProcRecursion;
use crate::keyword::Keyword;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::Parser;
use sappho_ast::{Ast, Expr, ListExpr, ProcExpr};
use sappho_ast_core::{
    BoxExpr, CommentedExpr, LetClause, LetExpr, MatchClause, MatchExpr, ProcEffect,
};

pub(crate) fn recursive_expr(
    expr: ProcRecursion,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    use Expr::List;

    list_expr(expr.clone())
        .map(List)
        .or(let_expr(expr.clone()).map(Expr::from))
        .or(match_expr(expr).map(Expr::from))
        .map(CommentedExpr::new_bare)
}

fn list_expr(
    expr: ProcRecursion,
) -> impl Parser<char, ListExpr<ProcEffect>, Error = BareError> + '_ {
    use crate::listform::list_form;

    list_form(expr.clone(), expr.map(BoxExpr::from))
        .map(ListExpr::new)
        .labelled("list-expression")
}

fn let_expr(
    expr: ProcRecursion,
) -> impl Parser<char, LetExpr<Ast, ProcEffect>, Error = BareError> + '_ {
    let_clause(expr.clone())
        .then_ignore(ws())
        .repeated()
        .at_least(1)
        .then(expr)
        .map(|(clauses, tail)| LetExpr {
            clauses,
            tail: BoxExpr::from(tail),
        })
        .labelled("let-expression")
}

fn let_clause(
    expr: ProcRecursion,
) -> impl Parser<char, LetClause<Ast, ProcEffect>, Error = BareError> + '_ {
    Keyword::Let
        .parser()
        .ignore_then(pattern())
        .then_ignore(just('=').delimited_by(ws(), ws()))
        .then(expr.clone())
        .then_ignore(just(';'))
        .map(|(binding, bindexpr)| LetClause {
            binding,
            bindexpr: BoxExpr::from(bindexpr),
        })
}

fn match_expr(
    expr: ProcRecursion,
) -> impl Parser<char, MatchExpr<Ast, ProcEffect>, Error = BareError> + '_ {
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
            target: BoxExpr::from(target),
            clauses,
        })
        .labelled("match-expression")
}

fn match_clause(
    expr: ProcRecursion,
) -> impl Parser<char, MatchClause<Ast, ProcEffect>, Error = BareError> + '_ {
    pattern()
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(expr)
        .map(|(pattern, body)| MatchClause {
            pattern,
            body: BoxExpr::from(body),
        })
        .labelled("match-clause")
}
