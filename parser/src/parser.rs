use crate::Error;
use chumsky::error::Simple;
use chumsky::text::digits;
use chumsky::Parser;
use saplang_ast::{Expr, Literal};
use std::str::FromStr;

pub fn expr() -> impl Parser<char, Expr, Error = Error> {
    literal().map(Expr::Lit)
}

fn literal() -> impl Parser<char, Literal, Error = Error> {
    number().map(Literal::Num)
}

fn number() -> impl Parser<char, f64, Error = Error> {
    digits(10).try_map(|digs: String, span| {
        f64::from_str(&digs).map_err(|e| Simple::custom(span, e.to_string()))
    })
}
