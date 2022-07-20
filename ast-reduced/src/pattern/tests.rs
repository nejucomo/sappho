use crate::{Literal, Pattern, UnpackPattern};
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
fn ast_to_red<const K: usize>(body: [&str; K], tail: Option<&str>) -> Pattern {
    use ast::Pattern::Bind;

    Pattern::from(ast::ListPattern::new(
        body.map(|s| Bind(s.to_string())),
        tail.map(|s| s.to_string()),
    ))
}

fn alp_new<'a, I>(bindpats: I, tailbind: Option<&str>) -> ast::Pattern
where
    I: IntoIterator<Item = &'a str>,
{
    ast::ListPattern::new(
        bindpats
            .into_iter()
            .map(|s| ast::Pattern::Bind(s.to_string())),
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
        Pattern::LitEq(Literal::Num(42.0)),
    )
    => ast::Pattern::Unpack(
        ast::UnpackPattern::from_iter([
            (
                "head".to_string(),
                ast::Pattern::Bind("a".to_string()),
            ),
            (
                "tail".to_string(),
                ast::Pattern::LitEq(ast::Literal::Num(42.0)),
            )
        ]),
    )
)]
fn red_to_ast(p: Pattern) -> ast::Pattern {
    p.into()
}
