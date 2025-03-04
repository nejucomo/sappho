mod effect;
mod object;
mod pattern;
mod proc;
mod recursive;
mod universal;

use chumsky::Parser;
use sappho_ast::{ProcExpr, PureExpr, QueryExpr};
use sappho_ast_core::CmtExpr;

use crate::comment::cmt_prefixed;
use crate::error::BareError;
use crate::restrict::Restrict;
use crate::RecExpr;

use self::proc::proc_expr_def;

pub(crate) fn expression() -> impl Parser<char, PureExpr, Error = BareError> {
    use chumsky::primitive::end;
    use chumsky::recursive::recursive;

    recursive(proc_expr_def)
        .try_map(PureExpr::restrict)
        .then_ignore(end())
}

fn proc_expr(rec: RecExpr<'_>) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    cmt_prefixed(rec.map(Box::new)).map(CmtExpr::from)
}

fn query_expr(rec: RecExpr<'_>) -> impl Parser<char, QueryExpr, Error = BareError> + '_ {
    proc_expr(rec).try_map(QueryExpr::restrict)
}

fn pure_expr(rec: RecExpr<'_>) -> impl Parser<char, PureExpr, Error = BareError> + '_ {
    proc_expr(rec).try_map(PureExpr::restrict)
}
