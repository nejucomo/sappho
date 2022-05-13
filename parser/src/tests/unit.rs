use saplang_ast::{
    Expr::{self, List, Lit, Ref},
    Literal::Num,
};
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
    Expr::let_expr(
        "x".to_string(),
        Lit(Num(42.0)),
        Ref("x".to_string()),
    )
    ; "let x x"
)]
#[test_case(
    "fn x -> x" =>
    Expr::func_expr(
        "x".to_string(),
        Ref("x".to_string()),
    )
    ; "identify fn"
)]
#[test_case(
    "f x" =>
    Expr::application(
        Ref("f".to_string()),
        Ref("x".to_string()),
    )
    ; "application"
)]
#[test_case(
    "f x y" =>
    Expr::application(
        Expr::application(
            Ref("f".to_string()),
            Ref("x".to_string()),
        ),
        Ref("y".to_string()),
    )
    ; "subsequent application"
)]
#[test_case(
    "g (f x)" =>
    Expr::application(
        Ref("g".to_string()),
        Expr::application(
            Ref("f".to_string()),
            Ref("x".to_string()),
        ),
    )
    ; "rightwards application"
)]
fn positive(input: &str) -> Expr {
    crate::parse(input).unwrap()
}
