pub trait FConvFrom<S>: Into<S> {
    fn fconv_from(src: S) -> Result<Self, S>;

    fn fconv_from_via<U>(src: S) -> Result<Self, S>
    where
        U: FConvFrom<S>,
        Self: FConvFrom<U>,
    {
        U::fconv_from(src).and_then(|u| Self::fconv_from(u).map_err(|u| u.into()))
    }
}

pub trait FConvInto<T>: From<T> {
    fn fconv_into(self) -> Result<T, Self>;

    fn fconv_into_via<U>(self) -> Result<T, Self>
    where
        Self: FConvInto<U>,
        U: FConvInto<T>,
    {
        self.fconv_into()
            .and_then(|u: U| u.fconv_into().map_err(Self::from))
    }
}

impl<S, T> FConvFrom<S> for T
where
    S: FConvInto<T>,
{
    fn fconv_from(src: S) -> Result<Self, S> {
        src.fconv_into()
    }
}
