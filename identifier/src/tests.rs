use test_case::test_case;

use sappho_keyword::Keyword::*;

use crate::{
    InvalidityReason::{self, *},
    RcId,
};

#[test_case("" => Err(Empty); "empty-string")]
#[test_case(" " => Err(ForbiddenInitialChar(' ')); "just-space")]
#[test_case("x " => Err(ForbiddenChar(' ')); "space-after-ident")]
#[test_case("7x" => Err(ForbiddenInitialChar('7')); "initial-digit")]
#[test_case("fn" => Err(ReservedKeyword(Fn)); "reserved-keyword-fn")]
#[test_case("let" => Err(ReservedKeyword(Let)); "reserved-keyword-let")]
#[test_case("match" => Err(ReservedKeyword(Match)); "reserved-keyword-match")]
#[test_case("query" => Err(ReservedKeyword(Query)); "reserved-keyword-query")]
#[test_case("proc" => Err(ReservedKeyword(Proc)); "reserved-keyword-proc")]
#[test_case("x" => Ok(()))]
#[test_case("procedure" => Ok(()))]
#[test_case("x7" => Ok(()))]
#[test_case("_" => Ok(()); "swallow")]
#[test_case("__" => Ok(()); "double_under")]
#[test_case("_ignored" => Ok(()); "swallow-named")]
#[test_case("foo_bar" => Ok(()))]
fn fromstr(s: &str) -> Result<(), InvalidityReason> {
    RcId::try_from(s.to_string())
        .map(|_| ())
        .map_err(|e| e.reason)
}
