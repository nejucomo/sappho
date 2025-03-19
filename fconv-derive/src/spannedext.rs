use syn::spanned::Spanned;

pub trait SpannedExt: Spanned {
    fn err<T, M>(&self, msg: M) -> syn::Result<T>
    where
        M: std::fmt::Display,
    {
        Err(self.error(msg))
    }

    fn error<M>(&self, msg: M) -> syn::Error
    where
        M: std::fmt::Display,
    {
        syn::Error::new(self.span(), msg)
    }
}

impl<T> SpannedExt for T where T: Spanned {}
