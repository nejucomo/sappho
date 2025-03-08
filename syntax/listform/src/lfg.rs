use derive_new::new;
use either::Either::{self, Left, Right};

use crate::ListFormIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, new)]
#[new(visibility = "pub(crate)")]
pub(crate) struct ListFormGeneric<XS, T> {
    pub(crate) xs: XS,
    pub(crate) optail: Option<T>,
}

impl<XS, T> ListFormGeneric<XS, T> {
    pub(crate) fn try_from_iter<I, X>(iter: I) -> Result<Self, Either<X, T>>
    where
        I: IntoIterator<Item = Either<X, T>>,
        XS: Default + Extend<X>,
    {
        let mut xs = XS::default();
        let mut optail = None;

        for ei in iter {
            if optail.is_none() {
                match ei {
                    Left(x) => xs.extend(Some(x)),
                    Right(t) => optail = Some(t),
                }
            } else {
                return Err(ei);
            }
        }

        Ok(ListFormGeneric { xs, optail })
    }

    pub(crate) fn as_ref(&self) -> ListFormGeneric<&XS, &T> {
        ListFormGeneric {
            xs: &self.xs,
            optail: self.optail.as_ref(),
        }
    }
}

impl<XS, T> Default for ListFormGeneric<XS, T>
where
    XS: Default,
{
    fn default() -> Self {
        ListFormGeneric::new(XS::default(), None)
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

impl<X, T> TryFrom<Vec<Either<X, T>>> for ListFormGeneric<Vec<X>, T> {
    /// The error is the first item following a tail:
    type Error = Either<X, T>;

    fn try_from(v: Vec<Either<X, T>>) -> Result<Self, Self::Error> {
        Self::try_from_iter(v)
    }
}
