use sappho_ast_core::Literal;
use sappho_ast_reduced as red;
use sappho_ast_rich as rich;
use sappho_attrs::Attrs;
use sappho_syntax_idstore::resolve_static;
use test_case::test_case;

use crate::TransformInto;

fn bind(s: &'static str) -> red::Pattern {
    red::Pattern::Bind(resolve_static(s))
}

fn unpack_empty() -> red::Pattern {
    red::Pattern::Unpack(Attrs::default())
}

fn cons_pat(head: &'static str, tail: red::Pattern) -> red::Pattern {
    Attrs::from_iter([
        (resolve_static("head"), bind(head)),
        (resolve_static("tail"), tail),
    ])
    .into()
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
fn rich_to_red<const K: usize>(
    body: [&'static str; K],
    tail: Option<&'static str>,
) -> red::Pattern {
    rich::ListPattern::new(
        body.map(resolve_static).map(rich::Pattern::Bind),
        tail.map(resolve_static),
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
            .map(resolve_static)
            .map(rich::Pattern::Bind),
        tailbind.map(resolve_static),
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
        red::Pattern::LitEq(Literal::Num(42.0)),
    )
    => rich::Pattern::Unpack(
        Attrs::from_iter([
            (
                resolve_static("head"),
                rich::Pattern::Bind(resolve_static("a")),
            ),
            (
                resolve_static("tail"),
                rich::Pattern::LitEq(Literal::Num(42.0)),
            )
        ]),
    )
)]
fn red_to_rich(p: red::Pattern) -> rich::Pattern {
    p.transform()
}
