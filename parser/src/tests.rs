use saplang_ast::{
    Application,
    Expr::{self, *},
    FuncExpr, LetExpr,
    Literal::*,
};
use std::rc::Rc;
use test_case::test_case;

#[test_case("42" => Lit(Num(42.0)))]
#[test_case("42\n" => Lit(Num(42.0)))]
#[test_case("bob" => Ref("bob".to_string()))]
#[test_case("bob  \n   " => Ref("bob".to_string()))]
#[test_case("[]" => List(vec![]) ; "tight empty list")]
#[test_case("[\n]" => List(vec![]) ; "multiline empty list")]
#[test_case("[ ] " => List(vec![]) ; "space empty list")]
#[test_case(
    "[42]" =>
    List(vec![
        Lit(Num(42.0))
    ])
    ; "tight singleton list"
)]
#[test_case(
    "[\n  42\n]" =>
    List(vec![
        Lit(Num(42.0))
    ])
    ; "multiline singleton list"
)]
#[test_case(
    "[42,bob]" =>
    List(vec![
        Lit(Num(42.0)),
        Ref("bob".to_string()),
    ])
    ; "tight pair list"
)]
#[test_case(
    "[42, bob]" =>
    List(vec![
        Lit(Num(42.0)),
        Ref("bob".to_string()),
    ])
    ; "natural pair list"
)]
#[test_case(
    "let x = 42;\nx" =>
    Let(Box::new(LetExpr {
        binding: "x".to_string(),
        bindexpr: Lit(Num(42.0)),
        tail: Ref("x".to_string()),
    }))
    ; "let x x"
)]
#[test_case(
    "fn x -> x" =>
    Func(FuncExpr {
        binding: "x".to_string(),
        body: Rc::new(Ref("x".to_string())),
    })
    ; "identify fn"
)]
#[test_case(
    "f x" =>
    Apply(Box::new(Application {
        target: Ref("f".to_string()),
        argument: Ref("x".to_string()),
    }))
    ; "application"
)]
#[test_case(
    "f x y" =>
    Apply(Box::new(Application {
        target: Apply(Box::new(Application {
            target: Ref("f".to_string()),
            argument: Ref("x".to_string()),
        })),
        argument: Ref("y".to_string()),
    }))
    ; "subsequent application"
)]
#[test_case(
    "g (f x)" =>
    Apply(Box::new(Application {
        target: Ref("g".to_string()),
        argument: Apply(Box::new(Application {
            target: Ref("f".to_string()),
            argument: Ref("x".to_string()),
        })),
    }))
    ; "rightwards application"
)]
fn positive(input: &str) -> Expr {
    crate::parse(input).unwrap()
}
