use std::iter::Fuse;

use derive_new::new;
use either::Either::{self, Left, Right};

use crate::lfg::ListFormGeneric;

#[derive(Clone, Debug, new)]
#[new(visibility = "pub(crate)")]
pub struct ListFormIter<XI, T>(ListFormGeneric<Fuse<XI>, T>);

impl<XI, T> Iterator for ListFormIter<XI, T>
where
    XI: Iterator,
{
    type Item = Either<XI::Item, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .xs
            .next()
            .map(Left)
            .or_else(|| self.0.optail.take().map(Right))
    }
}

impl<XI, T> DoubleEndedIterator for ListFormIter<XI, T>
where
    XI: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .optail
            .take()
            .map(Right)
            .or(self.0.xs.next_back().map(Left))
    }
}
