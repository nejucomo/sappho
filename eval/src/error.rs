use derive_more::From;
use sappho_east::{Identifier, Pattern};
use sappho_value::{CoercionFailure, Unbound, ValRef};
use std::fmt;

#[derive(Debug, From)]
pub enum Error {
    Unbound(Unbound),
    MissingAttr(ValRef, Identifier),
    Mismatch(ValRef, Vec<Pattern>),
    CoercionFailure(CoercionFailure),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Unbound(x) => x.fmt(f),
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
            CoercionFailure(x) => x.fmt(f),
        }
    }
}
