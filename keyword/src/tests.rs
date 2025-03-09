use sappho_parsable::Parser;
use test_case::test_case;

use crate::Keyword::{self, *};

#[test_case(Fn)]
#[test_case(Let)]
#[test_case(Match)]
#[test_case(Proc)]
#[test_case(Query)]
#[test_case(Return)]
fn parse_from_as_str_identity(kw: Keyword) {
    let s = kw.as_str();

    let res1 = Keyword::try_from(s);
    let res2 = kw.parse().load_and_parse(s).map_err(|_| ());
    assert_eq!(res1, res2);
}
