use crate::delimited::delimited;
use crate::error::BareError;
use crate::error::Span;
use crate::expr::pattern::pattern;
use crate::expr::universal::identifier;
use crate::expr::{pure_expr, query_expr};
use crate::keyword::Keyword;
use crate::space::ws;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{Expr, FuncDef, Identifier, ObjectDef, ProcExpr, QueryDef};
use sappho_ast_core::ProcEffects;

pub(crate) fn object_expr(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    use Expr::{Func, Query};

    object_def(expr.clone())
        .map(ProcExpr::from)
        .or(func_def(expr.clone()).map(Func))
        .or(query_def(expr).map(Query))
}

fn func_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, FuncDef, Error = BareError> + '_ {
    Keyword::Fn
        .parser()
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(pure_expr(expr))
        .map(|(binding, body)| FuncDef {
            binding,
            body: Box::new(body),
        })
        .labelled("fn definition")
}

fn query_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, QueryDef, Error = BareError> + '_ {
    Keyword::Query
        .parser()
        .ignore_then(query_expr(expr))
        .map(|body| QueryDef {
            body: Box::new(body),
        })
        .labelled("query definition")
}

fn object_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ObjectDef<ProcEffects>, Error = BareError> + '_ {
    let innards = object_clause(expr)
        .separated_by(just(',').then(ws().or_not()))
        .allow_trailing();

    delimited('{', innards, '}')
        .try_map(construct_object)
        .labelled("object definition")
}

enum ObjectClause {
    Attr(Identifier, ProcExpr),
    Func(FuncDef),
    Query(QueryDef),
}

fn object_clause(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ObjectClause, Error = BareError> + '_ {
    use ObjectClause::*;

    attr_def(expr.clone())
        .map(|(id, x)| Attr(id, x))
        .or(func_def(expr.clone()).map(Func))
        .or(query_def(expr).map(Query))
}

fn attr_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, (Identifier, ProcExpr), Error = BareError> + '_ {
    identifier()
        .then_ignore(ws().or_not())
        .then_ignore(just(':'))
        .then_ignore(ws().or_not())
        .then(expr)
        .labelled("attribute definition")
}

fn construct_object(
    clauses: Vec<ObjectClause>,
    span: Span,
) -> Result<ObjectDef<ProcEffects>, BareError> {
    use sappho_identmap::{IdentMap, RedefinitionError};

    let mut query = None;
    let mut func = None;
    let proc = None; // TODO: parse procs.
    let mut attrs = IdentMap::default();

    for clause in clauses.into_iter() {
        use ObjectClause::*;

        let clspan = span.clone();
        match clause {
            Query(x) => set_clause(&mut query, x, "query", clspan)?,
            Func(x) => set_clause(&mut func, x, "fn", clspan)?,
            Attr(id, x) => attrs.define(id, x).map_err(|RedefinitionError(id)| {
                BareError::custom(clspan, format!("duplicate attribute {:?}", id))
            })?,
        }
    }

    Ok(ObjectDef::new(func, query, proc, attrs))
}

fn set_clause<T>(
    slot: &mut Option<T>,
    clause: T,
    label: &str,
    span: Span,
) -> Result<(), BareError> {
    if slot.replace(clause).is_none() {
        Ok(())
    } else {
        Err(BareError::custom(
            span,
            format!("Object may not contain multiple {} clauses", label),
        ))
    }
}
