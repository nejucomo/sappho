use either::Either::{self, Left, Right};
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
    let lf: ListForm<_, _> = xs
        .into_iter()
        .map(Left)
        .chain(optail.into_iter().map(Right))
        .collect();

    let v: Vec<_> = lf.clone().into_iter().map(Either::into_inner).collect();
    assert_eq!(v.as_slice(), expected.as_slice());

    let v: Vec<_> = lf.into_iter().rev().map(Either::into_inner).collect();
    expected.as_mut_slice().reverse();
    assert_eq!(v.as_slice(), expected.as_slice());
}
