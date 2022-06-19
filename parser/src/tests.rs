use sappho_ast::{GenExpr, QueryEffects::Inquire};
use test_case::test_case;

#[test_case("42" => GenExpr::num(42.0) ; "forty-two")]
#[test_case("42\n" => GenExpr::num(42.0) ; "forty-two newline")]
#[test_case("bob" => GenExpr::ref_expr("bob".to_string()) ; "ref bob")]
#[test_case("bob  \n   " => GenExpr::ref_expr("bob".to_string()) ; "ref bob newline")]
#[test_case("[]" => GenExpr::list(vec![]) ; "tight empty list")]
#[test_case("[\n]" => GenExpr::list(vec![]) ; "multiline empty list")]
#[test_case("[ ] " => GenExpr::list(vec![]) ; "space empty list")]
#[test_case(
    "[42]" =>
    GenExpr::list(vec![
        GenExpr::num(42.0)
    ])
    ; "tight singleton list"
)]
#[test_case(
    "[\n  42\n]" =>
    GenExpr::list(vec![
        GenExpr::num(42.0)
    ])
    ; "multiline singleton list"
)]
#[test_case(
    "[42,bob]" =>
    GenExpr::list(vec![
        GenExpr::num(42.0),
        GenExpr::ref_expr("bob".to_string()),
    ])
    ; "tight pair list"
)]
#[test_case(
    "[42, bob]" =>
    GenExpr::list(vec![
        GenExpr::num(42.0),
        GenExpr::ref_expr("bob".to_string()),
    ])
    ; "natural pair list"
)]
#[test_case(
    "let x = 42; x" =>
    GenExpr::let_expr(
        "x".to_string(),
        GenExpr::num(42.0),
        GenExpr::ref_expr("x".to_string()),
    )
    ; "let x x space"
)]
#[test_case(
    "let x = 42;\nx" =>
    GenExpr::let_expr(
        "x".to_string(),
        GenExpr::num(42.0),
        GenExpr::ref_expr("x".to_string()),
    )
    ; "let x x newline"
)]
#[test_case(
    "fn x -> x" =>
    GenExpr::func_expr((
        "x".to_string(),
        GenExpr::ref_expr("x".to_string()),
    ))
    ; "identify fn"
)]
#[test_case(
    "f x" =>
    GenExpr::application(
        GenExpr::ref_expr("f".to_string()),
        GenExpr::ref_expr("x".to_string()),
    )
    ; "application"
)]
#[test_case(
    "f x y" =>
    GenExpr::application(
        GenExpr::application(
            GenExpr::ref_expr("f".to_string()),
            GenExpr::ref_expr("x".to_string()),
        ),
        GenExpr::ref_expr("y".to_string()),
    )
    ; "subsequent application"
)]
#[test_case(
    "g (f x)" =>
    GenExpr::application(
        GenExpr::ref_expr("g".to_string()),
        GenExpr::application(
            GenExpr::ref_expr("f".to_string()),
            GenExpr::ref_expr("x".to_string()),
        ),
    )
    ; "rightwards application"
)]
#[test_case(
    "query x" =>
    GenExpr::query_expr(GenExpr::ref_expr("x".to_string()))
    ; "query x"
)]
#[test_case(
    "query $x" =>
    GenExpr::query_expr(
        GenExpr::effect(
            Inquire(
                Box::new(GenExpr::ref_expr("x".to_string()))
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
        Some(GenExpr::ref_expr("x".to_string())),
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
            GenExpr::ref_expr("x".to_string()),
        )),
    )
    ; "object fn"
)]
#[test_case(
    "{ query x, fn x -> x }" =>
    GenExpr::object_expr(
        Some(GenExpr::ref_expr("x".to_string())),
        Some((
            "x".to_string(),
            GenExpr::ref_expr("x".to_string()),
        )),
    )
    ; "object query and fn"
)]
#[test_case(
    "{ fn x -> x, query x }" =>
    GenExpr::object_expr(
        Some(GenExpr::ref_expr("x".to_string())),
        Some((
            "x".to_string(),
            GenExpr::ref_expr("x".to_string()),
        )),
    )
    ; "object fn and query"
)]
#[test_case(
    "x.a" =>
    GenExpr::lookup(
        GenExpr::ref_expr("x".to_string()),
        "a".to_string(),
    )
    ; "x dot a"
)]
#[test_case(
    "x.a.b" =>
    GenExpr::lookup(
        GenExpr::lookup(
            GenExpr::ref_expr("x".to_string()),
            "a".to_string(),
        ),
        "b".to_string(),
    )
    ; "x dot a dot b"
)]
#[test_case(
    "f x.a" =>
    GenExpr::application(
        GenExpr::ref_expr("f".to_string()),
        GenExpr::lookup(
            GenExpr::ref_expr("x".to_string()),
            "a".to_string(),
        ),
    )
    ; "f applied to the a of x"
)]
#[test_case(
    "f (x.a)" =>
    GenExpr::application(
        GenExpr::ref_expr("f".to_string()),
        GenExpr::lookup(
            GenExpr::ref_expr("x".to_string()),
            "a".to_string(),
        ),
    )
    ; "f applied to the a of x with disambiguating parentheses"
)]
#[test_case(
    "f (x).a" =>
    GenExpr::application(
        GenExpr::ref_expr("f".to_string()),
        GenExpr::lookup(
            GenExpr::ref_expr("x".to_string()),
            "a".to_string(),
        ),
    )
    ; "f applied to the a of x with confusing parentheses"
)]
#[test_case(
    "(f x).a" =>
    GenExpr::lookup(
        GenExpr::application(
            GenExpr::ref_expr("f".to_string()),
            GenExpr::ref_expr("x".to_string()),
        ),
        "a".to_string(),
    )
    ; "the a of f applied to x with disambiguating parentheses"
)]
fn positive(input: &str) -> sappho_ast::PureExpr {
    match crate::parse(input) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            panic!()
        }
    }
}
