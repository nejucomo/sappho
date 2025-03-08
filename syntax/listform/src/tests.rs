use either::Either::{self, Left, Right};
use indoc::indoc;
use sappho_syntax_unparse::{Stream, Unparse};
use test_case::test_case;

use crate::ListForm;

#[test_case(["x", "y"], None, ["x", "y"])]
#[test_case(["x", "y"], Some("z"), ["x", "y", "z"])]
fn test_iter<I, const K: usize>(
    xs: I,
    optail: Option<&'static str>,
    mut expected: [&'static str; K],
) where
    I: IntoIterator<Item = &'static str>,
{
    let lf = ListForm::try_from_iter(
        xs.into_iter()
            .map(Left)
            .chain(optail.into_iter().map(Right)),
    )
    .unwrap();

    let v: Vec<_> = lf.clone().into_iter().map(Either::into_inner).collect();
    assert_eq!(v.as_slice(), expected.as_slice());

    let v: Vec<_> = lf.into_iter().rev().map(Either::into_inner).collect();
    expected.as_mut_slice().reverse();
    assert_eq!(v.as_slice(), expected.as_slice());
}

struct X;

impl Unparse for X {
    fn unparse_into(&self, s: &mut Stream) {
        s.write("X");
    }
}

#[test_case([], None => "[]")]
#[test_case([], Some(X) => indoc! { "
        [
          ..X
        ]"
    })]
#[test_case([X], None => indoc! { "
        [
          X
        ]"
    })]
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
    ListForm::new(body, tail).to_string()
}
