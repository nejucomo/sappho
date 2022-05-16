use crate::Error;
use chumsky::error::Simple;
use chumsky::{text, Parser};
use saplang_ast::{Identifier, Literal, UniversalExpr};
use std::str::FromStr;

pub(super) fn universal() -> impl Parser<char, UniversalExpr, Error = Error> {
    use UniversalExpr::{Lit, Ref};

    reference().map(Ref).or(literal().map(Lit))
}

fn reference() -> impl Parser<char, Identifier, Error = Error> {
    text::ident()
}

fn literal() -> impl Parser<char, Literal, Error = Error> {
    number().map(Literal::Num)
}

fn number() -> impl Parser<char, f64, Error = Error> {
    text::digits(10).try_map(|digs: String, span| {
        f64::from_str(&digs).map_err(|e| Simple::custom(span, e.to_string()))
    })
}
