use test_case::test_case;

use crate::RcId;

#[test_case("" => Err(()); "empty-string")]
#[test_case(" " => Err(()); "just-space")]
#[test_case("x " => Err(()); "space-after-ident")]
#[test_case("7x" => Err(()); "initial-digit")]
#[test_case("fn" => Err(()); "reserved-keyword-fn")]
#[test_case("let" => Err(()); "reserved-keyword-let")]
#[test_case("match" => Err(()); "reserved-keyword-match")]
#[test_case("query" => Err(()); "reserved-keyword-query")]
#[test_case("proc" => Err(()); "reserved-keyword-proc")]
#[test_case("x" => Ok(()))]
#[test_case("procedure" => Ok(()))]
#[test_case("x7" => Ok(()))]
#[test_case("_" => Ok(()); "swallow")]
#[test_case("__" => Ok(()); "double_under")]
#[test_case("_ignored" => Ok(()); "swallow-named")]
#[test_case("foo_bar" => Ok(()))]
fn fromstr(s: &str) -> Result<(), ()> {
    RcId::try_from(s.to_string()).map(|_| ()).map_err(|_| ())
}
