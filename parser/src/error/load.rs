use crate::error::Errors;
use std::fmt;

#[derive(Debug, derive_more::From)]
pub enum LoadParseError<'a> {
    Load(anyhow::Error),
    Parse(Errors<'a>),
}

impl<'a> fmt::Display for LoadParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LoadParseError::*;

        match self {
            Load(e) => {
                write!(f, "load error: ")?;
                e.fmt(f)
            }
            Parse(e) => {
                write!(f, "parse error: ")?;
                e.fmt(f)
            }
        }
    }
}
