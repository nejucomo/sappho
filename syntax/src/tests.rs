use sappho_parsable::load_and_parse;
// use sappho_regression_vectors as regression;
use test_case::test_case;

use crate::PureExpr;

const EMPTY_LIST: &[i32; 0] = &[];

#[test_case("42", &42; "forty-two")]
#[test_case("42\n", &42; "forty-two newline")]
#[test_case("bob", "bob"; "ref bob")]
#[test_case("bob  \n   ", "bob"; "ref bob newline")]
#[test_case("[]", EMPTY_LIST; "tight empty list")]
#[test_case("[\n]", EMPTY_LIST; "multiline empty list")]
#[test_case("[ ] ", EMPTY_LIST; "space empty list")]
#[test_case("[42]", &[42]; "tight singleton list")]
#[test_case("[\n  42\n]", &[42]; "multiline singleton list" )]
// #[test_case(
//     "[42,bob]" =>
//     list([
//         num(42.0),
//         refexpr("bob"),
//     ])
//     ; "tight pair list"
// )]
// #[test_case(
//     "[42, bob]" =>
//     list([
//         num(42.0),
//         refexpr("bob"),
//     ])
//     ; "natural pair list"
// )]
// #[test_case(
//     "let x = 42; x" =>
//     let_expr(
//         [(
//             bind("x"),
//             num(42.0),
//         )],
//         refexpr("x"),
//     )
//     ; "let x x space"
// )]
// #[test_case(
//     "let x = 42;\nx" =>
//     let_expr(
//         [(
//             bind("x"),
//             num(42.0),
//         )],
//         refexpr("x"),
//     )
//     ; "let x x newline"
// )]
// #[test_case(
//     "fn x -> x" =>
//     func_def_expr(
//         bind("x"),
//         refexpr("x"),
//     )
//     ; "identify fn"
// )]
// #[test_case(
//     "f x" =>
//     app_expr(
//         refexpr("f"),
//         refexpr("x"),
//     )
//     ; "application"
// )]
// #[test_case(
//     "f x y" =>
//     app_expr(
//         app_expr(
//             refexpr("f"),
//             refexpr("x"),
//         ),
//         refexpr("y"),
//     )
//     ; "subsequent app_expr"
// )]
// #[test_case(
//     "g (f x)" =>
//     app_expr(
//         refexpr("g"),
//         app_expr(
//             refexpr("f"),
//             refexpr("x"),
//         ),
//     )
//     ; "rightwards app_expr"
// )]
// #[test_case(
//     "query x" =>
//     query_def_expr(refexpr("x"))
//     ; "query x"
// )]
// #[test_case(
//     "query $x" =>
//     query_def_expr(
//         inquire(
//             refexpr("x")
//         )
//     )
//     ; "query inquire x"
// )]
// #[test_case(
//     "{}" =>
//     attrs_def([])
//     ; "empty object"
// )]
// #[test_case(
//     "{ a: x, b: x }" =>
//     attrs_def([("a", refexpr("x")), ("b", refexpr("x"))])
//     ; "attrs-only object single line spacey"
// )]
// #[test_case(
//     regression::UNPACK_MISSING_ATTRS =>
//     let_expr([
//         (
//             unpack_pat([
//                 ("a", "x"),
//                 ("b", "y"),
//                 ("c", "z"),
//             ]),
//             attrs_def([
//                 ("a", num(2.0)),
//             ])
//         )],
//         refexpr("z")
//     )
//     ; "attrs-only object unpack missing attrs integration regression"
// )]
// #[test_case(
//     "{ query x }" =>
//     object_def(
//         None,
//         Some(query_def(refexpr("x"))),
//     )
//     ; "object query"
// )]
// #[test_case(
//     "{ fn x -> x }" =>
//     object_def(
//         Some(func_def(
//             bind("x"),
//             refexpr("x"),
//         )),
//         None,
//     )
//     ; "object fn"
// )]
// #[test_case(
//     "{ query x, fn x -> x }" =>
//     object_def(
//         Some(func_def(
//             bind("x"),
//             refexpr("x"),
//         )),
//         Some(query_def(refexpr("x"))),
//     )
//     ; "object query and fn"
// )]
// #[test_case(
//     "{ fn x -> x, query x }" =>
//     object_def(
//         Some(func_def(
//             bind("x"),
//             refexpr("x"),
//         )),
//         Some(query_def(refexpr("x"))),
//     )
//     ; "object fn and query"
// )]
// #[test_case(
//     "x.a" =>
//     lookup(
//         refexpr("x"),
//         "a",
//     )
//     ; "x dot a"
// )]
// #[test_case(
//     "x.a.b" =>
//     lookup(
//         lookup(
//             refexpr("x"),
//             "a",
//         ),
//         "b",
//     )
//     ; "x dot a dot b"
// )]
// #[test_case(
//     "f x.a" =>
//     app_expr(
//         refexpr("f"),
//         lookup(
//             refexpr("x"),
//             "a",
//         ),
//     )
//     ; "f applied to the a of x"
// )]
// #[test_case(
//     "f (x.a)" =>
//     app_expr(
//         refexpr("f"),
//         lookup(
//             refexpr("x"),
//             "a",
//         ),
//     )
//     ; "f applied to the a of x with disambiguating parentheses"
// )]
// #[test_case(
//     "f (x).a" =>
//     app_expr(
//         refexpr("f"),
//         lookup(
//             refexpr("x"),
//             "a",
//         ),
//     )
//     ; "f applied to the a of x with confusing parentheses"
// )]
// #[test_case(
//     "(f x).a" =>
//     lookup(
//         app_expr(
//             refexpr("f"),
//             refexpr("x"),
//         ),
//         "a",
//     )
//     ; "the a of f applied to x with disambiguating parentheses"
// )]
// #[test_case(
//     "let [] = {}; 42" =>
//     let_expr(
//         [(
//             list_pat([], None),
//             attrs_def([]),
//         )],
//         num(42.0),
//     )
//     ; "let list empty"
// )]
// #[test_case(
//     "let [x] = {head: 42, tail: {}}; x" =>
//     let_expr(
//         [(
//             list_pat([bind("x")], None),
//             attrs_def([
//                 ("head", num(42.0)),
//                 ("tail", attrs_def([])),
//             ]),
//         )],
//         refexpr("x"),
//     )
//     ; "let list singleton"
// )]
// #[test_case(
//     "let [x, y] = {head: 2, tail: {head: 3, tail: {}}}; {a: x, b: y}" =>
//     let_expr(
//         [(
//             list_pat([bind("x"), bind("y")], None),
//             attrs_def([
//                 ("head", num(2.0)),
//                 ("tail", attrs_def([
//                     ("head", num(3.0)),
//                     ("tail", attrs_def([])),
//                 ]))
//             ]),
//         )],
//         attrs_def([
//             ("a", refexpr("x")),
//             ("b", refexpr("y")),
//         ]),
//     )
//     ; "let list pair"
// )]
// #[test_case(
//     "let [..t] = 42; t" =>
//     let_expr(
//         [(
//             list_pat([], Some("t")),
//             num(42.0),
//         )],
//         refexpr("t"),
//     )
//     ; "let list tail"
// )]
// #[test_case(
//     "let [h, ..t] = 42; {head: h, tail: t}" =>
//     let_expr(
//         [(
//             list_pat([bind("h")], Some("t")),
//             num(42.0),
//         )],
//         attrs_def([
//             ("head", refexpr("h")),
//             ("tail", refexpr("t")),
//         ]),
//     )
//     ; "let list singleton and tail"
// )]
fn parse_pure_expr<T>(input: &str, expected: &T)
where
    PureExpr: PartialEq<T>,
    T: ?Sized + std::fmt::Debug,
{
    let actual = load_and_parse::<PureExpr, _>(input).unwrap();
    assert_eq!(actual, *expected)
}
