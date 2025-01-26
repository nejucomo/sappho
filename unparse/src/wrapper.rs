use crate::{Error, Result, Stream, Unparse, UnparseWrappable};

#[derive(Debug)]
pub struct Wrapper<W>(W);

impl<W> Unparse for Wrapper<W>
where
    W: UnparseWrappable,
{
    fn unparse<S>(&self, stream: &mut S) -> Result<()>
    where
        S: Stream,
    {
        let mut trial = stream.position().wrap_trial();
        let res = self.0.unparse_wrappable(&mut trial, false);
        let fit = translate_error_to_fit(res)?;
        let wrap = !fit && self.0.should_wrap_if_unfit();
        self.0.unparse_wrappable(stream, wrap)
    }
}

fn translate_error_to_fit(r: Result<()>) -> Result<bool> {
    r.map(|()| true).or_else(|e| match e {
        Error::Wrap(_) => Ok(false),
        other => Err(other),
    })
}
