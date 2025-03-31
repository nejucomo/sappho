use sappho_attrs::Attrs;
use sappho_effect::QueryEffect;
use sappho_effect::{Effect, PureEffect};
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_parsable::{ParsableWith, Parser as _};
use sappho_regression_vectors as regression;
use sappho_tfi::TryFromIterator as _;
use test_case::test_case;

use crate::{
    Applications, BoxWise, Expr, FuncDef, Interactions, Let, Lookups, ObjectDef, ParensExpr,
    PureExpr, QueryDef, Wise,
};

fn w<T, FX>(value: T) -> Wise<FX>
where
    T: Into<Expr<FX>>,
    FX: Effect,
{
    Wise::new(value, None)
}

fn list_<X, T>() -> ListForm<X, T>
where
    X: std::fmt::Debug,
{
    list(())
}

fn list<S, X, T>(src: S) -> ListForm<X, T>
where
    S: Into<ListForm<X, T>>,
    X: std::fmt::Debug,
{
    src.into()
}

fn attrs_<T>() -> Attrs<T> {
    Attrs::default()
}

fn attrs<I, K, S, T>(items: I) -> Attrs<T>
where
    I: IntoIterator<Item = (K, S)>,
    RcId: TryFrom<K>,
    sappho_attrs::AttrsError: From<<RcId as TryFrom<K>>::Error>,
    S: Into<T>,
{
    Attrs::try_from_iterator(items.into_iter().map(|(k, v)| (k, v.into()))).unwrap()
}

#[test_case("42", 42; "forty-two")]
#[test_case("42\n", 42; "forty-two newline")]
#[test_case("bob", "bob"; "ref bob")]
#[test_case("bob  \n   ", "bob"; "ref bob newline")]
#[test_case("[]", list_(); "tight empty list")]
#[test_case("[\n]", list_(); "multiline empty list")]
#[test_case("[ ] ", list_(); "space empty list")]
#[test_case("[42]", list([w(42)]); "tight singleton list")]
#[test_case("[\n  42\n]", list([w(42i32)]); "multiline singleton list")]
#[test_case("[42,bob]", list((w(42), w("bob"))); "tight pair list")]
#[test_case("[42, bob]", list((w(42), w("bob"))); "natural pair list")]
#[test_case(
    "let x = 42; x",
    Let::new([("x", w(42))], w("x"))
    ; "let x x space"
)]
#[test_case(
    "let x = 42;\nx",
    Let::new([("x", w(42))], w("x"))
    ; "let x x newline"
)]
#[test_case(
    "fn x -> x",
    FuncDef::new("x", w("x"))
    ; "identify fn"
)]
#[test_case(
    "f x",
    Applications::new("f", ["x"])
    ; "applications-f-of-x"
)]
#[test_case(
    "f x y",
    Applications::new("f", ["x", "y"])
    ; "applications-f-of-x-then-y"
)]
#[test_case(
    "g (f x)",
    Applications::new("g", [ParensExpr::from(Applications::new("f", ["x"]))])
    ; "applications-g-of-f-of-x"
)]
#[test_case(
    "query x",
    QueryDef::new(w("x"))
    ; "query x"
)]
#[test_case(
    "query $x",
    QueryDef::new(w(Interactions::new(vec![QueryEffect::Inquire], "x")))
    ; "query inquire x"
)]
#[test_case(
    "{}",
    attrs_()
    ; "empty object"
)]
#[test_case(
    "{ a: x, b: x }",
    attrs([("a", w("x")), ("b", w("x"))])
    ; "attrs-only object single line spacey"
)]
#[test_case(
    "{ query x }",
    ObjectDef::from(QueryDef::new(w("x")))
    ; "object query"
)]
#[test_case(
    "{ fn x -> x }",
    ObjectDef::from(FuncDef::new("x", w("x")))
    ; "object fn"
)]
#[test_case(
    "{ query x, fn x -> x }",
    ObjectDef::new(FuncDef::new("x", w("x")), QueryDef::new(w("x")), None, attrs_())
    ; "object query and fn"
)]
#[test_case(
    "{ fn x -> x, query x }",
    ObjectDef::new(FuncDef::new("x", w("x")), QueryDef::new(w("x")), None, attrs_())
    ; "object fn and query"
)]
#[test_case(
    "x.a",
    Lookups::new("x", ["a"])
    ; "x dot a"
)]
#[test_case(
    "x.a.b",
    Lookups::new("x", ["a", "b"])
    ; "x dot a dot b"
)]
#[test_case(
    "f x.a",
    Applications::new("f", [Lookups::new("x", ["a"])])
    ; "f applied to the a of x"
)]
#[test_case(
    "f (x.a)",
    Applications::new("f", [ParensExpr::from(Lookups::new("x", ["a"]))])
    ; "f applied to the a of x with disambiguating parentheses"
)]
#[test_case(
    "f (x).a",
    Applications::new("f", [Lookups::new(ParensExpr::from("x"), ["a"])])
    ; "f applied to the a of x with confusing parentheses"
)]
#[test_case(
    "(f x).a",
    Lookups::new(Applications::new("f", ["x"]), ["a"])
    ; "the a of f applied to x with disambiguating parentheses"
)]
#[test_case(
    "let [] = {}; 42",
    Let::new([(list_(), w(attrs_()))], w(42))
    ; "let list empty"
)]
#[test_case(
    "let [x] = {head: 42, tail: {}}; x",
    Let::new(
        [(
            list(["x"]),
            w(attrs_()
                .with("head", w(42))
                .with("tail", w(attrs_())))
        )],
        w("x")
    )
    ; "let list singleton"
)]
#[test_case(
    "let [x, y] = {head: 2, tail: {head: 3, tail: {}}}; {a: x, b: y}",
    Let::new(
        [(
            list(["x", "y"]),
            w(attrs_()
                .with("head", w(2))
                .with(
                    "tail",
                    w(attrs_()
                        .with("head", w(3))
                        .with("tail", w(attrs_()))))
            )
        )],
        w(attrs([("a", w("x")), ("b", w("y"))]))
    )
    ; "let list pair"
)]
#[test_case(
    "let [..t] = 42; t",
    Let::new([(list_().with_tail("t"), w(42))], w("t"))
    ; "let list tail"
)]
#[test_case(
    "let [h, ..t] = 42; {head: h, tail: t}",
    Let::new(
        [(
            list(["h"]).with_tail("t"),
            w(42)
        )],
        w(attrs([
            ("head", w("h")),
            ("tail", w("t")),
        ]))
    )
    ; "let list singleton and tail"
)]
#[test_case(
    regression::UNPACK_MISSING_ATTRS,
    Let::new(
        [(
            attrs([
                ("a", "x"),
                ("b", "y"),
                ("c", "z"),
            ]),
            w(attrs([
                ("a", w(2)),
            ]))
        )],
        w("z")
    )
    ; "attrs-only object unpack missing attrs integration regression"
)]
fn parse_pure_expr<T>(input: &str, expected: T)
where
    T: Into<Expr<PureEffect>>,
{
    // Parse it with the source link `None`'d out:
    let actual = PureExpr::parser_with(None).load_and_parse(input).unwrap();
    assert_eq!(actual, BoxWise::new(expected, None));
}
