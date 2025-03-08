use sappho_syntax_parsable::{Parsable, Parser};
use test_case::test_case;

use crate::Keyword::{self, *};

#[test_case("fn" => Ok(Fn))]
#[test_case("let" => Ok(Let))]
#[test_case("match" => Ok(Match))]
#[test_case("proc" => Ok(Proc))]
#[test_case("query" => Ok(Query))]
#[test_case("return" => Ok(Return))]
#[test_case("def" => Err(()))]
// "non-shadowable bindings" are not keywords:
#[test_case("true" => Err(()))]
fn try_from_str(s: &str) -> Result<Keyword, ()> {
    let res1 = Keyword::try_from(s);
    let res2 = Keyword::parser().load_and_parse(s).map_err(|_| ());
    assert_eq!(res1, res2);
    res1
}
