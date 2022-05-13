use crate::Value::{self, Num};
use test_case::test_case;

#[test_case("42" => Num(42.0))]
fn eval(src: &str) -> Value {
    let vref = crate::eval(src).unwrap();
    std::rc::Rc::try_unwrap(vref).unwrap()
}
