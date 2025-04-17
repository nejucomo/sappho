use crate::error::BareError;
use crate::expr::pattern::pattern;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser as _;
use sappho_ast::{Ast, Expr, ListExpr, ProcExpr};
use sappho_ast_core::{LetClause, LetExpr, MatchClause, MatchExpr};
use sappho_ast_effect::ProcEffect;
use sappho_keyword::Keyword;
use sappho_parsable::primitive::space;
use sappho_parsable::Parser;

pub(crate) fn recursive_expr(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<ProcExpr> + '_ {
    use Expr::List;

    list_expr(expr.clone())
        .map(List)
        .or(let_expr(expr.clone()).map(Expr::from))
        .or(match_expr(expr).map(Expr::from))
}

fn list_expr(expr: Recursive<char, ProcExpr, BareError>) -> impl Parser<ListExpr<ProcEffect>> + '_ {
    use crate::listform::list_form;

    list_form(expr.clone(), expr.map(Box::new)).labelled("list-expression")
}

fn let_expr(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<LetExpr<Ast, ProcEffect>> + '_ {
    let_clause(expr.clone())
        .then_ignore(space())
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
) -> impl Parser<LetClause<Ast, ProcEffect>> + '_ {
    Keyword::Let
        .parse()
        .ignore_then(pattern())
        .then_ignore(just('=').delimited_by(space(), space()))
        .then(expr.clone())
        .then_ignore(just(';'))
        .map(|(binding, bindexpr)| LetClause {
            binding,
            bindexpr: Box::new(bindexpr),
        })
}

fn match_expr(
    expr: Recursive<char, ProcExpr, BareError>,
) -> impl Parser<MatchExpr<Ast, ProcEffect>> + '_ {
    use crate::delimited::delimited;

    Keyword::Match
        .parse()
        .ignore_then(expr.clone())
        .then_ignore(space())
        .then(delimited(
            '{',
            match_clause(expr)
                .separated_by(just(',').then(space()))
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
) -> impl Parser<MatchClause<Ast, ProcEffect>> + '_ {
    pattern()
        .then_ignore(just("->").delimited_by(space(), space()))
        .then(expr)
        .map(|(pattern, body)| MatchClause {
            pattern,
            body: Box::new(body),
        })
        .labelled("match-clause")
}
