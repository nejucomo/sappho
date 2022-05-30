use crate::error::BareError;
use chumsky::{text, Parser};
use sappho_ast::{Identifier, Literal, UniversalExpr};
use std::str::FromStr;

pub(super) fn universal_expr() -> impl Parser<char, UniversalExpr, Error = BareError> {
    use UniversalExpr::{Lit, Ref};

    reference().map(Ref).or(literal().map(Lit))
}

fn reference() -> impl Parser<char, Identifier, Error = BareError> {
    use crate::keyword::Keyword;

    text::ident()
        .try_map(|ident, span| {
            for kw in Keyword::iter() {
                if ident == kw.as_str() {
                    return Err(BareError::custom(
                        span,
                        format!("Keyword {:?} cannot be used as an identifier.", kw.as_str()),
                    ));
                }
            }

            Ok(ident)
        })
        .labelled("identifier reference")
}

fn literal() -> impl Parser<char, Literal, Error = BareError> {
    number().map(Literal::Num).labelled("literal")
}

fn number() -> impl Parser<char, f64, Error = BareError> {
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
