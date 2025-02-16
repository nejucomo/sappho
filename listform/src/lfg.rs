use derive_new::new;
use either::Either;

use crate::ListFormIter;

#[derive(Copy, Clone, Debug, PartialEq, new)]
#[new(visibility = "pub(crate)")]
pub(crate) struct ListFormGeneric<XS, T> {
    pub(crate) xs: XS,
    pub(crate) optail: Option<T>,
}

impl<XS, T> ListFormGeneric<XS, T> {
    pub fn as_ref(&self) -> ListFormGeneric<&XS, &T> {
        ListFormGeneric {
            xs: &self.xs,
            optail: self.optail.as_ref(),
        }
    }

    pub fn map_elem_container<F, YS>(self, f: F) -> ListFormGeneric<YS, T>
    where
        F: Fn(XS) -> YS,
    {
        ListFormGeneric {
            xs: f(self.xs),
            optail: self.optail,
        }
    }

    pub fn map_tail<F, U>(self, f: F) -> ListFormGeneric<XS, U>
    where
        F: Fn(T) -> U,
    {
        ListFormGeneric {
            xs: self.xs,
            optail: self.optail.map(f),
        }
    }
}

impl<XS, T> IntoIterator for ListFormGeneric<XS, T>
where
    XS: IntoIterator,
{
    type Item = Either<<XS::IntoIter as Iterator>::Item, T>;
    type IntoIter = ListFormIter<std::iter::Fuse<XS::IntoIter>, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListFormIter::new(self.map_elem_container(|xs| xs.into_iter().fuse()))
    }
}
