use crate::CorePattern;
use sappho_identmap::IdentMap;
use sappho_listform::ListForm;
use test_case::test_case;

fn bind(s: &str) -> CorePattern {
    CorePattern::Bind(s.to_string())
}

fn unpack_empty() -> CorePattern {
    CorePattern::Unpack(IdentMap::default())
}

fn cons_pat(head: &str, tail: CorePattern) -> CorePattern {
    IdentMap::from_iter([("head".to_string(), bind(head)), ("tail".to_string(), tail)]).into()
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
fn ast_to_core<const K: usize>(body: [&str; K], tail: Option<&str>) -> CorePattern {
    CorePattern::from(ListForm::new(body, tail))
}
