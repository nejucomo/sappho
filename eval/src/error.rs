use crate::ValRef;
use derive_more::From;
use sappho_ast::Identifier;
use std::fmt;

#[derive(Debug, From)]
pub enum Error {
    Unbound(Identifier),
    Uncallable(ValRef),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Unbound(id) => write!(f, "unbound {:?}", id),
            Uncallable(v) => write!(f, "not callable {:?}", v),
        }
    }
}
