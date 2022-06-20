use crate::delimited::delimited;
use crate::error::BareError;
use crate::expr::common::common_expr;
use crate::expr::effect::proc_effect;
use crate::expr::recursive::recursive_expr;
use crate::expr::universal::universal_expr;
use crate::space::ws;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::{Identifier, ProcExpr};

pub(super) fn proc_expr_def(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    non_application(pexpr)
        .separated_by(ws())
        .at_least(1)
        .map(|exprs| {
            exprs
                .into_iter()
                .reduce(ProcExpr::application)
                .expect(".at_least(1) postcondition failed.")
        })
}

fn non_application(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    non_app_non_lookup(pexpr)
        .then(attr_lookup().repeated())
        .map(|(x, lookups)| lookups.into_iter().fold(x, ProcExpr::lookup))
}

fn attr_lookup() -> impl Parser<char, Identifier, Error = BareError> {
    use crate::expr::universal::identifier;
    use chumsky::primitive::just;

    just('.').ignore_then(identifier())
}

fn non_app_non_lookup(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    parens_expr(pexpr.clone())
        .or(proc_effect(pexpr.clone()).map(ProcExpr::Effect))
        .or(universal_expr())
        .or(common_expr(pexpr.clone()))
        .or(recursive_expr(pexpr))
}

fn parens_expr(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    delimited('(', pexpr, ')').labelled("parenthetical-expression")
}
