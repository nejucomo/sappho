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
