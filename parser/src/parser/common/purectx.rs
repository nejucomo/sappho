use crate::parser::pattern::pattern;
use crate::parser::query_expr;
use crate::space::ws;
use crate::Error;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::{text, Parser};
use saplang_ast::{CommonExpr, FuncDef, PureExpr, QueryDef};

pub(crate) fn common_expr(
    expr: Recursive<'_, char, PureExpr, Error>,
) -> impl Parser<char, CommonExpr, Error = Error> + '_ {
    use CommonExpr::*;

    func_def(expr).map(Func).or(query_def().map(Query))
}

fn func_def(
    expr: Recursive<'_, char, PureExpr, Error>,
) -> impl Parser<char, FuncDef, Error = Error> + '_ {
    text::keyword("fn")
        .then_ignore(ws())
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(expr)
        .map(|(binding, body)| FuncDef {
            binding,
            body: Box::new(body),
        })
}

fn query_def() -> impl Parser<char, QueryDef, Error = Error> {
    text::keyword("query")
        .then_ignore(ws())
        .ignore_then(query_expr())
        .map(|body| QueryDef {
            body: Box::new(body),
        })
}
