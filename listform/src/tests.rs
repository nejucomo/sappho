use indoc::indoc;
use sappho_legible::{IntoNode, Legible, Node};
use test_case::test_case;

use crate::ListForm;

struct Displayer {
    lf: ListForm<X, X>,
}

impl IntoNode for &Displayer {
    fn into_node(self) -> Node {
        self.lf.into_node()
    }
}

impl std::fmt::Display for Displayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_width_threshold(f, 6)
    }
}

struct X;

impl IntoNode for &X {
    fn into_node(self) -> Node {
        "X".into_node()
    }
}

#[test_case([], None => "[]")]
#[test_case([], Some(X) => "[..X]")]
#[test_case([X], None => "[X]")]
#[test_case([X], Some(X) => indoc! { "
        [
          X,
          ..X
        ]"
    })]
#[test_case([X, X], Some(X) => indoc! { "
        [
          X,
          X,
          ..X
        ]"
    })]
fn display<const K: usize>(body: [X; K], tail: Option<X>) -> String {
    Displayer {
        lf: ListForm::new(body, tail),
    }
    .to_string()
}
