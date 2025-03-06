use sappho_syntax_parsable::{Parsable, Parser};
use test_case::test_case;

use crate::PrimVal::{self, Num};

#[test_case("42" => Some(Num(42.0)) ; "forty-two")]
#[test_case("42\n" => None ; "forty-two newline")]
fn parse(input: &str) -> Option<PrimVal> {
    PrimVal::parser().load_parse_source(input).ok()
}
