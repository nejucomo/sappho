use derive_more::From;
use std::fmt;

#[derive(Debug, From)]
pub enum Error {
    Std(std::io::Error),
    Parse(sappho_parser::Errors),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Std(e) => write!(f, "I/O error: {}", e),
            Parse(e) => e.fmt(f),
        }
    }
}
