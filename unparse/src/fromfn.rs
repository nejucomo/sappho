use crate::{Result, Stream, Unparse};

pub fn from_fn<F>(unparse: F) -> impl Unparse
where
    F: for<'a, 'b> Fn(&mut Stream<'a, 'b>) -> Result<bool>,
{
    FromFn(unparse)
}

pub struct FromFn<F>(F);

impl<F> Unparse for FromFn<F>
where
    F: for<'a, 'b> Fn(&mut Stream<'a, 'b>) -> Result<bool>,
{
    fn unparse<'a, 'b>(self, stream: &mut Stream<'a, 'b>) -> Result<bool> {
        self.0(stream)
    }
}
