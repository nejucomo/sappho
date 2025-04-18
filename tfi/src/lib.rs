pub trait TryFromIterator<T>: Sized + Default {
    type Error;

    fn try_from_iterator<I>(it: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let mut myself = Self::default();

        for item in it {
            myself = myself.try_append(item)?;
        }

        Ok(myself)
    }

    fn try_append(self, item: T) -> Result<Self, Self::Error>;
}

/// Extend iterators with [it.try_collect()](TryCollect::try_collect)
pub trait TryCollect: Sized + IntoIterator {
    fn try_collect<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self::Item>,
    {
        C::try_from_iterator(self)
    }
}

impl<I> TryCollect for I where I: IntoIterator {}
