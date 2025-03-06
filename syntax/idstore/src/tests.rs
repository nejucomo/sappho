use sappho_syntax_parsable::{Parsable, Parser};
use test_case::test_case;

use crate::{resolve_static, ArcId};

#[test_case("x" => Some(resolve_static("x")))]
#[test_case("true" => Some(resolve_static("true")))]
// This case should be the error: "Parse errors in <memory>:\n  Error 0: reserved keyword: \"fn\"" but there is a chumsky bug in `Simple`
#[test_case("fn" => None)]
fn parse(input: &str) -> Option<ArcId> {
    // This demonstrates a chumsky bug in error messages:
    // use chumsky::Parser;
    // dbg!(ArcId::parser().parse_recovery_verbose(input));

    ArcId::parser().load_parse_source(input).ok()
    // .map_err(|e| e.to_string())
}
