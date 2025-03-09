use std::fmt;

#[derive(Debug, derive_more::From)]
pub enum Error {
    LoadParse(sappho_parser::LoadParseError),
    Eval(sappho_eval::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
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
