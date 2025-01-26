use crate::{Error, Result, Stream};

pub trait UnparseContainer {
    fn unparse_container<'a, 'b>(&self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        let _ = stream;
        todo!()
        // let mut trial = stream.wrap_trial();
        // let res = self.unparse_wrapped(&mut trial, false);
        // let fit = translate_error_to_fit(res)?;
        // let wrap = !fit && self.should_wrap_if_unfit();
        // self.unparse_wrapped(stream, wrap)
    }

    fn unparse_wrapped<S>(&self, stream: &mut Stream, wrap: bool) -> Result<()>;

    fn should_wrap_if_unfit(&self) -> bool;
}

#[allow(dead_code)]
fn translate_error_to_fit(r: Result<()>) -> Result<bool> {
    r.map(|()| true).or_else(|e| match e {
        Error::Wrap(_) => Ok(false),
        other => Err(other),
    })
}
