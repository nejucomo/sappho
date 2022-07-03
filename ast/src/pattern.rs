mod list;
mod unpack;

use crate::{Identifier, Literal};
use std::fmt;

pub use self::list::ListPattern;
pub use self::unpack::UnpackPattern;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern {
    Bind(Identifier),
    LitEq(Literal),
    Unpack(UnpackPattern),
    List(ListPattern),
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Pattern::*;

        match self {
            Bind(x) => x.fmt(f),
            LitEq(x) => x.fmt(f),
            Unpack(x) => x.fmt(f),
            List(x) => x.fmt(f),
        }
    }
}
