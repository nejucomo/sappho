pub trait TryFromFunctional<S>: Into<S> + TryFrom<S, Error = S> {
    fn try_from_via<U>(src: S) -> Result<Self, S>
    where
        U: TryFromFunctional<S>,
        Self: TryFromFunctional<U>,
    {
        U::try_from(src).and_then(|u| Self::try_from(u).map_err(|u| u.into()))
    }
}

impl<T, S> TryFromFunctional<S> for T where T: Into<S> + TryFrom<S, Error = S> {}
