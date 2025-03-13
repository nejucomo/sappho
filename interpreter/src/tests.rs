use sappho_eval::ValRef;
use sappho_regression_vectors as regression;
use test_case::test_case;

fn errmsg(s: &str) -> Result<ValRef, String> {
    Err(s.to_string())
}

#[test_case("{ a: 2, b: 3 }" => matches Ok(_))]
#[test_case(regression::UNPACK_MISSING_ATTRS => errmsg("FIXME") ; "missing-attrs-on-unpack")]
fn interpret(source: &str) -> Result<ValRef, String> {
    crate::interpret(source).map_err(|e| e.to_string())
}
