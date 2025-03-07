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

impl<I, F> TryTransformInto<I> for F
where
    F: From<I>,
    I: TryTransformFrom<F>,
{
    fn try_transform_into(self) -> Either<I, Self> {
        I::try_transform_from(self)
    }
}

impl<F> TryTransformFrom<F> for F {
    fn try_transform_from(src: F) -> Either<Self, F> {
        Either::Left(src)
    }
}

impl<F> TryTransformFrom<Option<F>> for F {
    fn try_transform_from(src: Option<F>) -> Either<Self, Option<F>> {
        src.map(Left).unwrap_or(Right(None))
    }
}
