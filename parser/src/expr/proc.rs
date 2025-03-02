use crate::delimited::delimited;
use crate::error::BareError;
use crate::expr::effect::proc_effect;
use crate::expr::object::object_expr;
use crate::expr::recursive::recursive_expr;
use crate::expr::universal::universal_expr;
use crate::space::ws;
use chumsky::recursive::Recursive;
use chumsky::Parser;
use sappho_ast::ProcExpr;
use sappho_identifier::RcId;

pub(super) fn proc_expr_def(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    non_application(pexpr)
        .separated_by(ws())
        .at_least(1)
        .map(|exprs| {
            exprs
                .into_iter()
                .reduce(|t, a| {
                    use sappho_ast_core::ApplicationExpr;

                    ProcExpr::from(ApplicationExpr::new(Box::new(t), Box::new(a)))
                })
                .expect(".at_least(1) postcondition failed.")
        })
}

fn non_application(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    non_app_non_lookup(pexpr)
        .then(attr_lookup().repeated())
        .map(|(x, lookups)| {
            lookups.into_iter().fold(x, |x, attr| {
                use sappho_ast_core::LookupExpr;

                LookupExpr::new(Box::new(x), attr).into()
            })
        })
}

fn attr_lookup() -> impl Parser<char, RcId, Error = BareError> {
    use crate::expr::universal::identifier;
    use chumsky::primitive::just;

    just('.').ignore_then(identifier())
}

fn non_app_non_lookup(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    parens_expr(pexpr.clone())
        .or(proc_effect(pexpr.clone()).map(ProcExpr::from))
        .or(universal_expr())
        .or(object_expr(pexpr.clone()))
        .or(recursive_expr(pexpr))
}

fn parens_expr(
    pexpr: Recursive<'_, char, ProcExpr, BareError>,
) -> impl Parser<char, ProcExpr, Error = BareError> + '_ {
    delimited('(', pexpr, ')').labelled("parenthetical-expression")
}
