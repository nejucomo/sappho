use either::Either::{self, Left, Right};

pub trait TryTransformFrom<F>: Sized + Into<F> {
    fn try_transform_from(src: F) -> Either<Self, F>;

    fn try_transform_from_via<G>(f: F) -> Either<Self, F>
    where
        Self: TryTransformFrom<G>,
        G: TryTransformFrom<F>,
    {
        G::try_transform_from(f).left_and_then(|g| {
            <Self as TryTransformFrom<G>>::try_transform_from(g).map_right(G::into)
        })
    }
}

pub trait TryTransformInto<I>: Sized + From<I> {
    fn try_transform_into(self) -> Either<I, Self>;
}

impl<I, F> TryTransformFrom<F> for I
where
    F: From<I>,
    F: TryTransformInto<I>,
{
    fn try_transform_from(src: F) -> Either<Self, F> {
        src.try_transform_into()
    }
}

impl<F> TryTransformInto<F> for F {
    fn try_transform_into(self) -> Either<F, Self> {
        Left(self)
    }
}

impl<T, const K: usize> TryTransformInto<[T; K]> for Vec<T>
where
    T: std::fmt::Debug,
{
    fn try_transform_into(self) -> Either<[T; K], Self> {
        if self.len() == K {
            Left(<[T; K]>::try_from(self).unwrap())
        } else {
            Right(self)
        }
    }
}
