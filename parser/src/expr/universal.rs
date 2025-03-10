use crate::error::BareError;
use chumsky::{text, Parser as _};
use sappho_ast::ProcExpr;
use sappho_ast_core::Literal;
use sappho_identifier::RcId;
use sappho_parsable::{Parsable, Parser};
use std::str::FromStr;

pub(super) fn universal_expr() -> impl Parser<ProcExpr> {
    use sappho_ast_core::CoreExpr::{Lit, Ref};

    identifier()
        .map(Ref)
        .or(literal().map(Lit))
        .map(ProcExpr::from)
}

pub(super) fn identifier() -> impl Parser<RcId> {
    RcId::parser()
}

pub(super) fn literal() -> impl Parser<Literal> {
    number().map(Literal::Num).labelled("literal")
}

fn number() -> impl Parser<f64> {
    use chumsky::primitive::filter;

    let disallowed_trailing_char = filter(|&c: &char| c.is_alphabetic() || c.is_control())
        .try_map(|c, span| -> Result<(), BareError> {
            Err(BareError::custom(
                span,
                format!("unexpected {:?} in numeric literal", c),
            ))
        })
        .or_not();

    text::digits(10)
        .then_ignore(disallowed_trailing_char)
        .try_map(|digs: String, span| {
            f64::from_str(&digs).map_err(|e| BareError::custom(span, e.to_string()))
        })
        .labelled("number")
}
