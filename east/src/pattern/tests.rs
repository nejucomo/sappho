use crate::{Pattern, UnpackPattern};
use sappho_ast as ast;
use test_case::test_case;

fn bind(s: &str) -> Pattern {
    Pattern::Bind(s.to_string())
}

fn unpack_empty() -> Pattern {
    Pattern::Unpack(UnpackPattern::default())
}

fn cons_pat(head: &str, tail: Pattern) -> Pattern {
    UnpackPattern::from_iter([("head".to_string(), bind(head)), ("tail".to_string(), tail)]).into()
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
fn transform_list_pattern<const K: usize>(body: [&str; K], tail: Option<&str>) -> Pattern {
    use ast::Pattern::Bind;

    Pattern::from(ast::ListPattern::new(
        body.map(|s| Bind(s.to_string())),
        tail.map(|s| s.to_string()),
    ))
}
