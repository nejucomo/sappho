mod procdef;

use chumsky::primitive::just;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{Ast, Expr, ProcExpr};
use sappho_ast_core::{FuncDef, ObjectDef, ProcDef, QueryDef};
use sappho_ast_effect::ProcEffect;
use sappho_identifier::RcId;
use sappho_object::Element;

use crate::delimited::delimited;
use crate::error::BareError;
use crate::expr::pattern::pattern;
use crate::expr::universal::identifier;
use crate::expr::{pure_expr, query_expr};
use crate::keyword::Keyword;
use crate::space::ws;

use self::procdef::proc_def;

pub(crate) fn object_expr(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    use Expr::{Func, Proc, Query};

    object_def(expr.clone())
        .map(Expr::from)
        .or(func_def(expr.clone()).map(Func))
        .or(query_def(expr.clone()).map(Query))
        .or(proc_def(expr).map(Proc))
}

fn func_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, FuncDef<Ast>, Error = BareError> + '_ {
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
) -> impl Parser<char, QueryDef<Ast>, Error = BareError> + '_ {
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
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ObjectClause, Error = BareError> + '_ {
    use Element::*;

    attr_def(expr.clone())
        .map(|(id, x)| Attr(id, x))
        .or(func_def(expr.clone()).map(Func))
        .or(query_def(expr.clone()).map(Query))
        .or(proc_def(expr).map(Proc))
}

fn attr_def(
    expr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, (RcId, ProcExpr), Error = BareError> + '_ {
    identifier()
        .then_ignore(ws().or_not())
        .then_ignore(just(':'))
        .then_ignore(ws().or_not())
        .then(expr)
        .labelled("attribute definition")
}
