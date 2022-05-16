use crate::parser::pattern::pattern;
use crate::parser::pure_expr;
use crate::space::ws;
use crate::Error;
use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::{text, Parser};
use saplang_ast::{CommonExpr, FuncDef, QueryDef, QueryExpr};

pub(crate) fn common_expr(
    expr: Recursive<'_, char, QueryExpr, Error>,
) -> impl Parser<char, CommonExpr, Error = Error> + '_ {
    use CommonExpr::*;

    func_def().map(Func).or(query_def(expr).map(Query))
}

fn func_def() -> impl Parser<char, FuncDef, Error = Error> {
    text::keyword("fn")
        .then_ignore(ws())
        .ignore_then(pattern())
        .then_ignore(just("->").delimited_by(ws(), ws()))
        .then(pure_expr())
        .map(|(binding, body)| FuncDef {
            binding,
            body: Box::new(body),
        })
}

fn query_def(
    expr: Recursive<'_, char, QueryExpr, Error>,
) -> impl Parser<char, QueryDef, Error = Error> + '_ {
    text::keyword("query")
        .then_ignore(ws())
        .ignore_then(expr)
        .map(|body| QueryDef {
            body: Box::new(body),
        })
}
