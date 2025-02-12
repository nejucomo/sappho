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
use chumsky::Parser;
use sappho_ast::{ProcExpr, PureExpr, QueryExpr};

pub(crate) type ProcRecursion<'a> = Recursive<'a, char, ProcExpr, BareError>;

pub(crate) fn expression() -> impl Parser<char, PureExpr, Error = BareError> {
    use chumsky::primitive::end;
    use chumsky::recursive::recursive;

    recursive(proc_expr_def)
        .try_map(PureExpr::restrict)
        .then_ignore(end())
}

fn query_expr(pexpr: ProcRecursion<'_>) -> impl Parser<char, QueryExpr, Error = BareError> + '_ {
    pexpr.try_map(QueryExpr::restrict)
}

fn pure_expr(pexpr: ProcRecursion<'_>) -> impl Parser<char, PureExpr, Error = BareError> + '_ {
    pexpr.try_map(PureExpr::restrict)
}
