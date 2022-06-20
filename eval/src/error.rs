use crate::ValRef;
use derive_more::From;
use sappho_east::{Identifier, Pattern};
use std::fmt;

#[derive(Debug, From)]
pub enum Error {
    Unbound(Identifier),
    Uncallable(ValRef),
    MissingAttr(ValRef, Identifier),
    Mismatch(ValRef, Vec<Pattern>),
    CoercionFailure(ValRef, &'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Unbound(id) => write!(f, "unbound {:?}", id),
            Uncallable(v) => write!(f, "not callable {}", v),
            MissingAttr(v, name) => write!(f, "missing attr {}.{}", v, name),
            Mismatch(v, pats) => {
                write!(
                    f,
                    "value {} does not match any of these patterns: {}",
                    v,
                    pats.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            CoercionFailure(v, typename) => write!(f, "Could not coerce {} to {}", v, typename),
        }
    }
}
