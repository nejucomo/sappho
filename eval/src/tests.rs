use crate::List::Cell;
use crate::Value::{self, List, Num};
use test_case::test_case;

#[test_case(
    "42" =>
    matches Num(_)
    ; "Literal 42"
)]
#[test_case(
    "[42]" =>
    matches List(
        Cell(_, _)
    )
    ; "singleton list"
)]
#[test_case(
    "let x = 42; x" =>
    matches Num(_)
    ; "let and deref"
)]
#[test_case(
    "(fn x -> x) 42" =>
    matches Num(_)
    ; "fn and apply"
)]
fn eval(src: &str) -> Value {
    let ast = sappho_parser::parse(src).unwrap();
    let vref = crate::eval(ast).unwrap();
    vref.unwrap()
}
