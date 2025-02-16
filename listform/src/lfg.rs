use derive_new::new;
use either::Either::{self, Left, Right};

use crate::ListFormIter;

#[derive(Copy, Clone, Debug, PartialEq, new)]
#[new(visibility = "pub(crate)")]
pub(crate) struct ListFormGeneric<XS, T> {
    pub(crate) xs: XS,
    pub(crate) optail: Option<T>,
}

impl<XS, T> ListFormGeneric<XS, T> {
    pub(crate) fn as_ref(&self) -> ListFormGeneric<&XS, &T> {
        ListFormGeneric {
            xs: &self.xs,
            optail: self.optail.as_ref(),
        }
    }
}

impl<XS, T> IntoIterator for ListFormGeneric<XS, T>
where
    XS: IntoIterator,
{
    type Item = Either<<XS::IntoIter as Iterator>::Item, T>;
    type IntoIter = ListFormIter<XS::IntoIter, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListFormIter::new(ListFormGeneric {
            xs: self.xs.into_iter().fuse(),
            optail: self.optail,
        })
    }
}

/// # Panic
///
/// This panics if a `Right` is ever encountered in any position besides the last element.
///
/// Note: The std `impl<A, E, V> FromIterator<Result<A, E> for Result<V, E>` impl cannot help us here. :-(
impl<X, T> FromIterator<Either<X, T>> for ListFormGeneric<Vec<X>, T>
where
    X: std::fmt::Debug,
    T: std::fmt::Debug,
{
    fn from_iter<I: IntoIterator<Item = Either<X, T>>>(iter: I) -> Self {
        let mut xs = vec![];
        let mut optail = None;

        for ei in iter {
            // BUG: A better API would cause this to be an `Err` somehow:
            assert!(optail.is_none(), "out-of-order tail: {ei:?}");
            match ei {
                Left(x) => xs.push(x),
                Right(t) => optail = Some(t),
            }
        }

        ListFormGeneric { xs, optail }
    }
}
