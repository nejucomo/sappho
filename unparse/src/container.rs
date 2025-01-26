use crate::{Error, Result, Stream, Unparse};

pub trait UnparseContainer {
    fn unparse_container<'a, 'b>(&self, stream: &mut Stream<'a, 'b>) -> Result<()> {
        let mut trial = stream.wrap_trial();
        let res = self.unparse_wrapped(&mut trial, false);
        let fit = translate_error_to_fit(res)?;
        let wrap = !fit && self.should_wrap_if_unfit();
        self.unparse_wrapped(stream, wrap)
    }

    fn unparse_wrapped<'a, 'b>(&self, stream: &mut Stream<'a, 'b>, wrap: bool) -> Result<()> {
        self.unparse_header(stream)?;
        if wrap {
            stream.newline_indent(true)?;
        }
        let mut sep = false;
        for child in self.unparse_iter() {
            stream.write(&child)?;
            if sep {
                stream.write(Self::unparse_separator())?;
                if wrap {
                    stream.newline()?;
                } else {
                    stream.write(" ")?;
                }
            } else {
                sep = true
            }
        }
        self.unparse_footer(stream)
    }

    fn should_wrap_if_unfit(&self) -> bool {
        self.unparse_iter().count() > 1
    }

    fn unparse_header<'a, 'b>(&self, stream: &mut Stream<'a, 'b>) -> Result<()>;

    fn unparse_footer<'a, 'b>(&self, stream: &mut Stream<'a, 'b>) -> Result<()>;

    type UnparseChild<'s>: Unparse
    where
        Self: 's;

    fn unparse_iter<'s>(&'s self) -> impl Iterator<Item = Self::UnparseChild<'s>>;

    fn unparse_separator() -> &'static str;
}

fn translate_error_to_fit(r: Result<()>) -> Result<bool> {
    r.map(|()| true).or_else(|e| match e {
        Error::Wrap(_) => Ok(false),
        other => Err(other),
    })
}
