use saplang_ast::{
    GenExpr::{self, Effect, List, Lit, Ref},
    Literal::Num,
    QueryEffects,
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
    GenExpr::let_expr(
        "x".to_string(),
        Lit(Num(42.0)),
        Ref("x".to_string()),
    )
    ; "let x x"
)]
#[test_case(
    "fn x -> x" =>
    GenExpr::func_expr(
        "x".to_string(),
        Ref("x".to_string()),
    )
    ; "identify fn"
)]
#[test_case(
    "f x" =>
    GenExpr::application(
        Ref("f".to_string()),
        Ref("x".to_string()),
    )
    ; "application"
)]
#[test_case(
    "f x y" =>
    GenExpr::application(
        GenExpr::application(
            Ref("f".to_string()),
            Ref("x".to_string()),
        ),
        Ref("y".to_string()),
    )
    ; "subsequent application"
)]
#[test_case(
    "g (f x)" =>
    GenExpr::application(
        Ref("g".to_string()),
        GenExpr::application(
            Ref("f".to_string()),
            Ref("x".to_string()),
        ),
    )
    ; "rightwards application"
)]
#[test_case(
    "query x" =>
    GenExpr::query_expr(Ref("x".to_string()))
    ; "query x"
)]
#[test_case(
    "query $x" =>
    GenExpr::query_expr(
        Effect(
            QueryEffects::Inquire(
                Box::new(Ref("x".to_string()))
            )
        )
    )
    ; "query inquire x"
)]
#[test_case(
    "{}" =>
    GenExpr::object_expr(None, None)
    ; "empty object"
)]
#[test_case(
    "{ query x }" =>
    GenExpr::object_expr(
        Some(Ref("x".to_string())),
        None,
    )
    ; "object query"
)]
#[test_case(
    "{ fn x -> x }" =>
    GenExpr::object_expr(
        None,
        Some((
            "x".to_string(),
            Ref("x".to_string()),
        )),
    )
    ; "object fn"
)]
#[test_case(
    "{ query x; fn x -> x }" =>
    GenExpr::object_expr(
        Some(Ref("x".to_string())),
        Some((
            "x".to_string(),
            Ref("x".to_string()),
        )),
    )
    ; "object full"
)]
fn positive(input: &str) -> saplang_ast::Expr {
    crate::parse(input).unwrap()
}
