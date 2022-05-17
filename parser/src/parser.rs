mod common;
mod pattern;
mod proc;
mod procfx;
mod recursive;
mod universal;

use self::proc::proc_expr_def;
use crate::restrict::Restrict;
use crate::Error;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use saplang_ast::{ProcExpr, PureExpr, QueryExpr};

pub(crate) fn expression() -> impl Parser<char, PureExpr, Error = Error> {
    use chumsky::primitive::end;
    use chumsky::recursive::recursive;

    recursive(proc_expr_def)
        .try_map(PureExpr::restrict)
        .then_ignore(end())
}

fn query_expr(
    proc_expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, QueryExpr, Error = Error> + '_ {
    proc_expr.try_map(QueryExpr::restrict)
}

fn pure_expr(
    proc_expr: Recursive<'_, char, ProcExpr, Error>,
) -> impl Parser<char, PureExpr, Error = Error> + '_ {
    proc_expr.try_map(PureExpr::restrict)
}
