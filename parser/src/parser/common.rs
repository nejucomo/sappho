use crate::parser::pattern::pattern;
use crate::parser::{pure_expr, query_expr};
use crate::space::ws;
use crate::Error;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::{text, Parser};
use saplang_ast::{CommonExpr, FuncDef, ProcExpr, QueryDef};

pub(crate) fn common_expr(
    expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, CommonExpr, Error = Error> + '_ {
    use CommonExpr::*;

    func_def(expr.clone())
        .map(Func)
        .or(query_def(expr).map(Query))
}

fn func_def(
    expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, FuncDef, Error = Error> + '_ {
    text::keyword("fn")
        .then_ignore(ws())
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
    text::keyword("query")
        .then_ignore(ws())
        .ignore_then(query_expr(expr))
        .map(|body| QueryDef {
            body: Box::new(body),
        })
}
