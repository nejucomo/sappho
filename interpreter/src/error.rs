// TODO: use thiserror
use std::fmt;

#[derive(Debug, derive_more::From)]
pub enum Error<'a> {
    LoadParse(sappho_parser::LoadParseError<'a>),
    Eval(sappho_eval::Error),
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            LoadParse(e) => e.fmt(f),
            Eval(e) => {
                write!(f, "eval error: ")?;
                e.fmt(f)
            }
        }
    }
}
