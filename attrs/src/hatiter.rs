use either::Either::{self, Left, Right};

use crate::error::AttrsResult;
use crate::Attrs;

pub trait AttrsTailAdapter: Sized {
    fn into_attrs_tail(self) -> Either<Attrs<Self>, Self>;
}

#[derive(Clone, Debug)]
pub struct HeadAndTailIter<T>(Option<Either<Attrs<T>, T>>)
where
    T: AttrsTailAdapter;

impl<T> From<Attrs<T>> for HeadAndTailIter<T>
where
    T: AttrsTailAdapter,
{
    fn from(attrs: Attrs<T>) -> Self {
        HeadAndTailIter(Some(Left(attrs)))
    }
}

impl<T> HeadAndTailIter<T>
where
    T: AttrsTailAdapter,
{
    fn next_inner(&mut self) -> AttrsResult<Option<Either<T, T>>> {
        if let Some(ei) = self.0.take() {
            match ei {
                Left(attrs) => self.next_attrs(attrs),
                Right(tail) => Ok(Some(Right(tail))),
            }
        } else {
            Ok(None)
        }
    }

    fn next_attrs(&mut self, attrs: Attrs<T>) -> AttrsResult<Option<Either<T, T>>> {
        if attrs.is_empty() {
            Ok(None)
        } else {
            let [elem, tail] = attrs.unpack(["head", "tail"])?;
            self.0 = Some(tail.into_attrs_tail());
            Ok(Some(Left(elem)))
        }
    }
}

impl<T> Iterator for HeadAndTailIter<T>
where
    T: AttrsTailAdapter,
{
    type Item = AttrsResult<Either<T, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_inner().transpose()
    }
}
