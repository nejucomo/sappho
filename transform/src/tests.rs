//! Tests here assume the lower stack is correct, and thus rely on parse/unparse
use sappho_ast_core::Literal;
use sappho_ast_reduced as astred;
use sappho_ast_rich as rich;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_parser::parse;
use sappho_regression_vectors as regression;
use sappho_unparse::Unparse;
use test_case::test_case;

use crate::TransformInto;

fn bind(s: &'static str) -> astred::Pattern {
    astred::Pattern::Bind(RcId::from(s))
}

fn unpack_empty() -> astred::Pattern {
    astred::Pattern::Unpack(Attrs::default())
}

fn cons_pat(head: &'static str, tail: astred::Pattern) -> astred::Pattern {
    Attrs::from_iter([("head", bind(head)), ("tail", tail)]).into()
}

#[test_case(
    regression::UNPACK_MISSING_ATTRS,
    regression::UNPACK_MISSING_ATTRS
    ; "unpack-missing-attrs-regression"
)]
fn reduce(input: &str, expected: &str) {
    let astx: rich::PureExpr = parse(input).unwrap();
    dbg!(&astx);
    let expected = parse(expected).unwrap();
    dbg!(&expected);
    let actual = crate::reduce(astx);
    dbg!(&actual);
    assert_eq!(expected.unparse().to_string(), actual.unparse().to_string());
}

#[test_case([], None => unpack_empty())]
#[test_case(
    [],
    Some("t")
    => bind("t")
)]
#[test_case(
    ["a"],
    None
    => cons_pat(
        "a",
        unpack_empty(),
    )
)]
#[test_case(
    ["a"],
    Some("t")
    => cons_pat(
        "a",
        bind("t"),
    )
)]
#[test_case(
    ["a", "b"],
    Some("t")
    => cons_pat(
        "a",
        cons_pat(
            "b",
            bind("t"),
        ),
    )
)]
fn ast_to_red<const K: usize>(
    body: [&'static str; K],
    tail: Option<&'static str>,
) -> astred::Pattern {
    rich::ListPattern::new(
        body.map(RcId::from).map(rich::Pattern::Bind),
        tail.map(RcId::from),
    )
    .transform()
}

fn alp_new<I>(bindpats: I, tailbind: Option<&'static str>) -> rich::Pattern
where
    I: IntoIterator<Item = &'static str>,
{
    rich::ListPattern::new(
        bindpats
            .into_iter()
            .map(RcId::from)
            .map(rich::Pattern::Bind),
        tailbind.map(RcId::from),
    )
    .into()
}

#[test_case(
    unpack_empty()
    => alp_new([], None)
)]
#[test_case(
    cons_pat(
        "a",
        cons_pat(
            "b",
            bind("t"),
        ),
    )
    => alp_new(["a", "b"], Some("t"))
)]
#[test_case(
    cons_pat(
        "a",
        astred::Pattern::LitEq(Literal::Num(42.0)),
    )
    => rich::Pattern::Unpack(
        Attrs::from_iter([
            (
                "head",
                rich::Pattern::Bind(RcId::from("a")),
            ),
            (
                "tail",
                rich::Pattern::LitEq(Literal::Num(42.0)),
            )
        ]),
    )
)]
fn red_to_ast(p: astred::Pattern) -> rich::Pattern {
    p.transform()
}
