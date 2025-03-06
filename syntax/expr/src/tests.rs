use sappho_syntax_parsable::{Parsable, Parser};
use test_case::test_case;

use crate::{Expr, InnerExpr};

#[test_case("42" => Some(Expr::from(42)))]
#[test_case("(42)" => Some(Expr::from(InnerExpr::from(Expr::from(42)))))]
fn parse(input: &str) -> Option<Expr> {
    Expr::parser().load_parse_source(input).ok()
}
