use crate::Embed;

pub trait Extract<T>: Sized + Embed<T> {
    fn extract(self) -> Result<T, Self>;

    fn extract_via<U>(self) -> Result<T, Self>
    where
        Self: Extract<U>,
        U: Extract<T>,
    {
        self.extract()
            .and_then(|u: U| u.extract().map_err(Self::embed))
    }
}

impl<T> Extract<T> for T {
    fn extract(self) -> Result<T, Self> {
        Ok(self)
    }
}

impl<T> Extract<T> for Option<T> {
    fn extract(self) -> Result<T, Self> {
        self.ok_or(None)
    }
}

impl<T, E> Extract<T> for Result<T, E> {
    fn extract(self) -> Result<T, Self> {
        self.map_err(Err)
    }
}
