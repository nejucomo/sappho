use crate::delimited::delimited;
use crate::keyword::Keyword;
use crate::parser::pattern::pattern;
use crate::parser::{pure_expr, query_expr};
use crate::space::ws;
use crate::Error;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::{CommonExpr, FuncDef, ObjectDef, ProcExpr, QueryDef};

pub(crate) fn common_expr(
    expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, CommonExpr, Error = Error> + '_ {
    use CommonExpr::*;

    object_def(expr.clone())
        .map(Object)
        .or(func_def(expr.clone()).map(Func))
        .or(query_def(expr).map(Query))
}

fn func_def(
    expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, FuncDef, Error = Error> + '_ {
    Keyword::Fn
        .parser()
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(pure_expr(expr))
        .map(|(binding, body)| FuncDef {
            binding,
            body: Box::new(body),
        })
}

fn query_def(
    expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, QueryDef, Error = Error> + '_ {
    Keyword::Query
        .parser()
        .ignore_then(query_expr(expr))
        .map(|body| QueryDef {
            body: Box::new(body),
        })
}

fn object_def(
    expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ObjectDef, Error = Error> + '_ {
    let innards = object_clause(expr)
        .separated_by(just(';').then(ws().or_not()))
        .try_map(construct_object);

    delimited('{', innards, '}')
}

enum ObjectClause {
    Func(FuncDef),
    Query(QueryDef),
}

fn object_clause(
    expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, ObjectClause, Error = Error> + '_ {
    use ObjectClause::*;

    func_def(expr.clone())
        .map(Func)
        .or(query_def(expr).map(Query))
}

fn construct_object(
    clauses: Vec<ObjectClause>,
    span: <Error as chumsky::error::Error<char>>::Span,
) -> Result<ObjectDef, Error> {
    let mut query = None;
    let mut func = None;

    for clause in clauses.into_iter() {
        use ObjectClause::*;

        let clspan = span.clone();
        match clause {
            Query(x) => set_clause(&mut query, x, "query", clspan)?,
            Func(x) => set_clause(&mut func, x, "fn", clspan)?,
        }
    }

    Ok(ObjectDef { query, func })
}

fn set_clause<T>(
    slot: &mut Option<T>,
    clause: T,
    label: &str,
    span: <Error as chumsky::error::Error<char>>::Span,
) -> Result<(), Error> {
    if slot.replace(clause).is_none() {
        Ok(())
    } else {
        use chumsky::error::Simple;

        Err(Simple::custom(
            span,
            format!("Object may not contain multiple {} clauses", label),
        ))
    }
}
