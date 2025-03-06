use sappho_syntax_parsable::{Parsable, Parser};
use test_case::test_case;

use crate::Identifier;

#[test_case("x" => Some(Identifier::from_static("x")))]
#[test_case("true" => Some(Identifier::from_static("true")))]
// This case should be the error: "Parse errors in <memory>:\n  Error 0: reserved keyword: \"fn\"" but there is a chumsky bug in `Simple`
#[test_case("fn" => None)]
fn parse(input: &str) -> Option<Identifier> {
    // This demonstrates a chumsky bug in error messages:
    // use chumsky::Parser;
    // dbg!(Identifier::parser().parse_recovery_verbose(input));

    Identifier::parser().load_parse_source(input).ok()
    // .map_err(|e| e.to_string())
}
