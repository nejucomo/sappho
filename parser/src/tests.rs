use chumsky::Parser;
use saplang_ast::{Expr, Literal};
use test_case::test_case;

#[test_case("42" => Expr::Lit(Literal::Num(42.0)))]
fn parse(input: &str) -> Expr {
    crate::expr()
        .then_ignore(chumsky::primitive::end())
        .parse(input)
        .unwrap()
}
