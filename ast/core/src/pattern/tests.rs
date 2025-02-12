use crate::{CorePattern, Literal};
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
fn ast_to_red<const K: usize>(body: [&str; K], tail: Option<&str>) -> CorePattern {
    CorePattern::from(IdentMap::from_listform(
        ListForm::new(body, tail)
            .map_elems(|s| CorePattern::from(s.to_string()))
            .map_tail(|s| s.to_string()),
    ))
}

fn alp_new<'a, I>(bindpats: I, tailbind: Option<&str>) -> ast::CorePattern
where
    I: IntoIterator<Item = &'a str>,
{
    ast::CoreListPattern::new(
        bindpats
            .into_iter()
            .map(|s| ast::CorePattern::Bind(s.to_string())),
        tailbind.map(|s| s.to_string()),
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
        CorePattern::LitEq(Literal::Num(42.0)),
    )
    => ast::CorePattern::Unpack(
        ast::IdentMap::from_iter([
            (
                "head".to_string(),
                ast::CorePattern::Bind("a".to_string()),
            ),
            (
                "tail".to_string(),
                ast::CorePattern::LitEq(ast::Literal::Num(42.0)),
            )
        ]),
    )
)]
fn red_to_ast(p: CorePattern) -> ast::CorePattern {
    p.into()
}
