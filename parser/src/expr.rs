mod effect;
mod object;
mod pattern;
mod proc;
mod recursive;
mod universal;

use self::proc::proc_expr_def;
use crate::error::BareError;
use crate::restrict::Restrict;
use chumsky::recursive::Recursive;
use chumsky::Parser as _;
use sappho_ast::{ProcExpr, PureExpr, QueryExpr};
use sappho_parsable::Parser;

pub(crate) fn expression() -> impl Parser<PureExpr> {
    use chumsky::primitive::end;
    use chumsky::recursive::recursive;

    recursive(proc_expr_def)
        .try_map(PureExpr::restrict)
        .then_ignore(end())
}

fn query_expr(proc_expr: Recursive<'_, char, ProcExpr, BareError>) -> impl Parser<QueryExpr> + '_ {
    proc_expr.try_map(QueryExpr::restrict)
}

fn pure_expr(proc_expr: Recursive<'_, char, ProcExpr, BareError>) -> impl Parser<PureExpr> + '_ {
    proc_expr.try_map(PureExpr::restrict)
}
