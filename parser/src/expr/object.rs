mod procdef;

use self::procdef::proc_def;
use crate::delimited::delimited;
use crate::error::BareError;
use crate::expr::pattern::pattern;
use crate::expr::universal::identifier;
use crate::expr::{pure_expr, query_expr, ProcRecursion};
use crate::keyword::Keyword;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::Parser;
use sappho_ast::{Ast, Expr, Identifier, ProcExpr};
use sappho_ast_core::{BoxExpr, CommentedExpr, FuncDef, ObjectDef, ProcDef, ProcEffect, QueryDef};
use sappho_object::Element;

pub(crate) fn object_expr(
    expr: ProcRecursion<'_>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    use Expr::{Func, Proc, Query};

    object_def(expr.clone())
        .map(Expr::from)
        .map(CommentedExpr::new_bare)
        .or(func_def(expr.clone())
            .map(Func)
            .map(CommentedExpr::new_bare))
        .or(query_def(expr.clone())
            .map(Query)
            .map(CommentedExpr::new_bare))
        .or(proc_def(expr).map(Proc).map(CommentedExpr::new_bare))
}

fn func_def(expr: ProcRecursion<'_>) -> impl Parser<char, FuncDef<Ast>, Error = BareError> + '_ {
    Keyword::Fn
        .parser()
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(pure_expr(expr))
        .map(|(binding, body)| FuncDef {
            binding,
            body: BoxExpr::from(body),
        })
        .labelled("fn definition")
}

fn query_def(expr: ProcRecursion<'_>) -> impl Parser<char, QueryDef<Ast>, Error = BareError> + '_ {
    Keyword::Query
        .parser()
        .ignore_then(query_expr(expr))
        .map(|body| QueryDef {
            body: BoxExpr::from(body),
        })
        .labelled("query definition")
}

fn object_def(
    expr: ProcRecursion<'_>,
) -> impl Parser<char, ObjectDef<Ast, ProcEffect>, Error = BareError> + '_ {
    let innards = object_clause(expr)
        .separated_by(just(',').then(ws().or_not()))
        .allow_trailing();

    delimited('{', innards, '}')
        .try_map(|clauses, span| {
            clauses
                .into_iter()
                .collect::<Result<sappho_object::Object<_, _, _, _>, String>>()
                .map(ObjectDef::new)
                .map_err(|msg| BareError::custom(span, msg))
        })
        .labelled("object definition")
}

type ObjectClause = Element<FuncDef<Ast>, QueryDef<Ast>, ProcDef<Ast>, ProcExpr>;

fn object_clause(
    expr: ProcRecursion<'_>,
) -> impl Parser<char, ObjectClause, Error = BareError> + '_ {
    use Element::*;

    attr_def(expr.clone())
        .map(|(id, x)| Attr(id, x))
        .or(func_def(expr.clone()).map(Func))
        .or(query_def(expr.clone()).map(Query))
        .or(proc_def(expr).map(Proc))
}

fn attr_def(
    expr: ProcRecursion<'_>,
) -> impl Parser<char, (Identifier, ProcExpr), Error = BareError> + '_ {
    identifier()
        .then_ignore(ws().or_not())
        .then_ignore(just(':'))
        .then_ignore(ws().or_not())
        .then(expr)
        .labelled("attribute definition")
}
