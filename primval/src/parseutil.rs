use std::str::FromStr;

use chumsky::prelude::filter;
use chumsky::{text, Parser as _};
use sappho_parsable::Parser;

use crate::Num;

pub(crate) fn number() -> impl Parser<Num> {
    let disallowed_trailing_char = filter(|&c: &char| c.is_alphabetic() || c.is_control())
        .try_map_ez(|c| -> Result<(), _> { Err(format!("unexpected {c:?} in numeric literal")) })
        .or_not();

    text::digits(10)
        .then_ignore(disallowed_trailing_char)
        .try_map_ez(|digs: String| Num::from_str(&digs))
        .labelled("number")
}
