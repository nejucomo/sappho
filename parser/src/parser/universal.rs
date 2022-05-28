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

    text::ident().try_map(|ident, span| {
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
}

fn literal() -> impl Parser<char, Literal, Error = BareError> {
    number().map(Literal::Num)
}

fn number() -> impl Parser<char, f64, Error = BareError> {
    text::digits(10).try_map(|digs: String, span| {
        f64::from_str(&digs).map_err(|e| BareError::custom(span, e.to_string()))
    })
}
