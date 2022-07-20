use sappho_ast::{EffectExpr, Expr, FuncDef, ListPattern, Pattern, PureExpr, QueryDef, QueryExpr};
use sappho_ast_core::{ApplicationExpr, LetClause, LetExpr, LookupExpr, ObjectDef};
use sappho_identmap::IdentMap;
use test_case::test_case;

fn num(f: f64) -> PureExpr {
    sappho_ast_core::Literal::Num(f).into()
}

fn refexpr<FX>(s: &str) -> Expr<FX> {
    s.to_string().into()
}

fn bind(s: &str) -> Pattern {
    Pattern::Bind(s.to_string())
}

fn inquire(x: QueryExpr) -> QueryExpr {
    use sappho_ast_core::QueryEffects;
    QueryExpr::from(EffectExpr::new(QueryEffects::Inquire, Box::new(x)))
}

fn list<T>(xs: T) -> PureExpr
where
    T: IntoIterator<Item = PureExpr>,
{
    Expr::from_iter(xs)
}

fn let_expr<const K: usize>(clauses: [(Pattern, PureExpr); K], bindexpr: PureExpr) -> PureExpr {
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
    PureExpr::Func(func_def(p, x))
}

fn query_def(x: QueryExpr) -> QueryDef {
    QueryDef::new(Box::new(x))
}

fn query_def_expr(x: QueryExpr) -> PureExpr {
    PureExpr::Query(query_def(x))
}

fn object_def(f: Option<FuncDef>, q: Option<QueryDef>) -> PureExpr {
    ObjectDef::new(f, q, Default::default()).into()
}

fn attrs_def<const K: usize>(attrs: [(&str, PureExpr); K]) -> PureExpr {
    let stringattrs = attrs.into_iter().map(|(s, x)| (s.to_string(), x));
    ObjectDef::new(None, None, IdentMap::from_iter(stringattrs)).into()
}

fn app_expr(t: PureExpr, a: PureExpr) -> PureExpr {
    ApplicationExpr::new(Box::new(t), Box::new(a)).into()
}

fn lookup(t: PureExpr, attr: &str) -> PureExpr {
    LookupExpr::new(Box::new(t), attr.to_string()).into()
}

fn list_pat<const K: usize>(pats: [Pattern; K], tail: Option<&str>) -> Pattern {
    Pattern::List(ListPattern::new(pats, tail.map(|s| s.to_string())))
}

#[test_case("42" => num(42.0) ; "forty-two")]
#[test_case("42\n" => num(42.0) ; "forty-two newline")]
#[test_case("bob" => refexpr("bob") ; "ref bob")]
#[test_case("bob  \n   " => refexpr("bob") ; "ref bob newline")]
#[test_case("[]" => list([]) ; "tight empty list")]
#[test_case("[\n]" => list([]) ; "multiline empty list")]
#[test_case("[ ] " => list([]) ; "space empty list")]
#[test_case(
    "[42]" =>
    list([
        num(42.0)
    ])
    ; "tight singleton list"
)]
#[test_case(
    "[\n  42\n]" =>
    list([
        num(42.0)
    ])
    ; "multiline singleton list"
)]
#[test_case(
    "[42,bob]" =>
    list([
        num(42.0),
        refexpr("bob"),
    ])
    ; "tight pair list"
)]
#[test_case(
    "[42, bob]" =>
    list([
        num(42.0),
        refexpr("bob"),
    ])
    ; "natural pair list"
)]
#[test_case(
    "let x = 42; x" =>
    let_expr(
        [(
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
        [(
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
        inquire(
            refexpr("x")
        )
    )
    ; "query inquire x"
)]
#[test_case(
    "{}" =>
    attrs_def([])
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
#[test_case(
    "let [] = {}; 42" =>
    let_expr(
        [(
            list_pat([], None),
            attrs_def([]),
        )],
        num(42.0),
    )
    ; "let list empty"
)]
#[test_case(
    "let [x] = {head: 42, tail: {}}; x" =>
    let_expr(
        [(
            list_pat([bind("x")], None),
            attrs_def([
                ("head", num(42.0)),
                ("tail", attrs_def([])),
            ]),
        )],
        refexpr("x"),
    )
    ; "let list singleton"
)]
#[test_case(
    "let [x, y] = {head: 2, tail: {head: 3, tail: {}}}; {a: x, b: y}" =>
    let_expr(
        [(
            list_pat([bind("x"), bind("y")], None),
            attrs_def([
                ("head", num(2.0)),
                ("tail", attrs_def([
                    ("head", num(3.0)),
                    ("tail", attrs_def([])),
                ]))
            ]),
        )],
        attrs_def([
            ("a", refexpr("x")),
            ("b", refexpr("y")),
        ]),
    )
    ; "let list pair"
)]
#[test_case(
    "let [..t] = 42; t" =>
    let_expr(
        [(
            list_pat([], Some("t")),
            num(42.0),
        )],
        refexpr("t"),
    )
    ; "let list tail"
)]
#[test_case(
    "let [h, ..t] = 42; {head: h, tail: t}" =>
    let_expr(
        [(
            list_pat([bind("h")], Some("t")),
            num(42.0),
        )],
        attrs_def([
            ("head", refexpr("h")),
            ("tail", refexpr("t")),
        ]),
    )
    ; "let list singleton and tail"
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
