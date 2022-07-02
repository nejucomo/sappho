use sappho_ast::{
    FuncDef,
    GenExpr::{self, Effect},
    PureExpr, QueryDef,
    QueryEffects::Inquire,
    QueryExpr,
};
use sappho_gast::{ApplicationExpr, LetClause, LetExpr, LookupExpr, ObjectDef, Pattern};
use test_case::test_case;

fn num(f: f64) -> PureExpr {
    sappho_gast::Literal::Num(f).into()
}

fn refexpr<FX>(s: &str) -> GenExpr<FX> {
    s.to_string().into()
}

fn bind(s: &str) -> Pattern {
    Pattern::Bind(s.to_string())
}

fn list<T>(xs: T) -> PureExpr
where
    T: IntoIterator<Item = PureExpr>,
{
    GenExpr::from_iter(xs)
}

fn let_expr(clauses: Vec<(Pattern, PureExpr)>, bindexpr: PureExpr) -> PureExpr {
    LetExpr::new(
        clauses
            .into_iter()
            .map(|(p, x)| LetClause::new(p, Box::new(x)))
            .collect(),
        Box::new(bindexpr),
    )
    .into()
}

fn func_def(p: Pattern, x: PureExpr) -> FuncDef {
    FuncDef::new(p, Box::new(x))
}

fn func_def_expr(p: Pattern, x: PureExpr) -> PureExpr {
    func_def(p, x).into()
}

fn query_def(x: QueryExpr) -> QueryDef {
    QueryDef::new(Box::new(x))
}

fn query_def_expr(x: QueryExpr) -> PureExpr {
    query_def(x).into()
}

fn object_def(f: Option<FuncDef>, q: Option<QueryDef>) -> PureExpr {
    ObjectDef::new(f, q, Default::default()).into()
}

fn app_expr(t: PureExpr, a: PureExpr) -> PureExpr {
    ApplicationExpr::new(Box::new(t), Box::new(a)).into()
}

fn lookup(t: PureExpr, attr: &str) -> PureExpr {
    LookupExpr::new(Box::new(t), attr.to_string()).into()
}

#[test_case("42" => num(42.0) ; "forty-two")]
#[test_case("42\n" => num(42.0) ; "forty-two newline")]
#[test_case("bob" => refexpr("bob") ; "ref bob")]
#[test_case("bob  \n   " => refexpr("bob") ; "ref bob newline")]
#[test_case("[]" => list(vec![]) ; "tight empty list")]
#[test_case("[\n]" => list(vec![]) ; "multiline empty list")]
#[test_case("[ ] " => list(vec![]) ; "space empty list")]
#[test_case(
    "[42]" =>
    list(vec![
        num(42.0)
    ])
    ; "tight singleton list"
)]
#[test_case(
    "[\n  42\n]" =>
    list(vec![
        num(42.0)
    ])
    ; "multiline singleton list"
)]
#[test_case(
    "[42,bob]" =>
    list(vec![
        num(42.0),
        refexpr("bob"),
    ])
    ; "tight pair list"
)]
#[test_case(
    "[42, bob]" =>
    list(vec![
        num(42.0),
        refexpr("bob"),
    ])
    ; "natural pair list"
)]
#[test_case(
    "let x = 42; x" =>
    let_expr(
        vec![(
            bind("x"),
            num(42.0),
        )],
        refexpr("x"),
    )
    ; "let x x space"
)]
#[test_case(
    "let x = 42;\nx" =>
    let_expr(
        vec![(
            bind("x"),
            num(42.0),
        )],
        refexpr("x"),
    )
    ; "let x x newline"
)]
#[test_case(
    "fn x -> x" =>
    func_def_expr(
        bind("x"),
        refexpr("x"),
    )
    ; "identify fn"
)]
#[test_case(
    "f x" =>
    app_expr(
        refexpr("f"),
        refexpr("x"),
    )
    ; "application"
)]
#[test_case(
    "f x y" =>
    app_expr(
        app_expr(
            refexpr("f"),
            refexpr("x"),
        ),
        refexpr("y"),
    )
    ; "subsequent app_expr"
)]
#[test_case(
    "g (f x)" =>
    app_expr(
        refexpr("g"),
        app_expr(
            refexpr("f"),
            refexpr("x"),
        ),
    )
    ; "rightwards app_expr"
)]
#[test_case(
    "query x" =>
    query_def_expr(refexpr("x"))
    ; "query x"
)]
#[test_case(
    "query $x" =>
    query_def_expr(
        Effect(
            Inquire(
                Box::new(refexpr("x"))
            )
        )
    )
    ; "query inquire x"
)]
#[test_case(
    "{}" =>
    object_def(None, None)
    ; "empty object"
)]
#[test_case(
    "{ query x }" =>
    object_def(
        None,
        Some(query_def(refexpr("x"))),
    )
    ; "object query"
)]
#[test_case(
    "{ fn x -> x }" =>
    object_def(
        Some(func_def(
            bind("x"),
            refexpr("x"),
        )),
        None,
    )
    ; "object fn"
)]
#[test_case(
    "{ query x, fn x -> x }" =>
    object_def(
        Some(func_def(
            bind("x"),
            refexpr("x"),
        )),
        Some(query_def(refexpr("x"))),
    )
    ; "object query and fn"
)]
#[test_case(
    "{ fn x -> x, query x }" =>
    object_def(
        Some(func_def(
            bind("x"),
            refexpr("x"),
        )),
        Some(query_def(refexpr("x"))),
    )
    ; "object fn and query"
)]
#[test_case(
    "x.a" =>
    lookup(
        refexpr("x"),
        "a",
    )
    ; "x dot a"
)]
#[test_case(
    "x.a.b" =>
    lookup(
        lookup(
            refexpr("x"),
            "a",
        ),
        "b",
    )
    ; "x dot a dot b"
)]
#[test_case(
    "f x.a" =>
    app_expr(
        refexpr("f"),
        lookup(
            refexpr("x"),
            "a",
        ),
    )
    ; "f applied to the a of x"
)]
#[test_case(
    "f (x.a)" =>
    app_expr(
        refexpr("f"),
        lookup(
            refexpr("x"),
            "a",
        ),
    )
    ; "f applied to the a of x with disambiguating parentheses"
)]
#[test_case(
    "f (x).a" =>
    app_expr(
        refexpr("f"),
        lookup(
            refexpr("x"),
            "a",
        ),
    )
    ; "f applied to the a of x with confusing parentheses"
)]
#[test_case(
    "(f x).a" =>
    lookup(
        app_expr(
            refexpr("f"),
            refexpr("x"),
        ),
        "a",
    )
    ; "the a of f applied to x with disambiguating parentheses"
)]
fn positive(input: &str) -> PureExpr {
    match crate::parse(input) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            panic!()
        }
    }
}
