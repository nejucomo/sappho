use derive_more::From;
use sappho_east::{Identifier, Pattern};
use sappho_value::{BindFailure, CoercionFailure, Unbound, ValRef};
use std::fmt;

/// An evaluation error.
#[derive(Debug, From)]
pub enum Error {
    Unbound(Unbound),
    MissingAttr(ValRef, Identifier),
    Mismatch(ValRef, Vec<Pattern>),
    CoercionFailure(CoercionFailure),
    BindFailure(BindFailure),
}

/// An alias for a [std::result::Result] with the eval [Error] type.
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Unbound(x) => x.fmt(f),
            MissingAttr(v, name) => write!(f, "missing attr {}.{}", v, name),
            Mismatch(v, pats) => {
                use sappho_fmtutil::fmt_comma_sep;

                write!(f, "value {} does not match any of these patterns: ", v)?;
                fmt_comma_sep(pats, f)
            }
            CoercionFailure(x) => x.fmt(f),
            BindFailure(x) => x.fmt(f),
        }
    }
}
