use chumsky::primitive::filter;
use chumsky::{text, Parser as _};
use sappho_syntax_parsable::error::ChumskyError;
use sappho_syntax_parsable::Parser;

pub(crate) fn number() -> impl Parser<f64> {
    let disallowed_trailing_char = filter(|&c: &char| c.is_alphabetic() || c.is_control())
        .try_map(|c, span| -> Result<(), ChumskyError> {
            Err(ChumskyError::custom(
                span,
                format!("unexpected {:?} in numeric literal", c),
            ))
        })
        .or_not();

    text::digits(10)
        .then_ignore(disallowed_trailing_char)
        .try_map(|digs: String, span| {
            <f64 as std::str::FromStr>::from_str(&digs)
                .map_err(|e| ChumskyError::custom(span, e.to_string()))
        })
        .labelled("number")
}
