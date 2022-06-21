use crate::{Identifier, Literal};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Bind(Identifier),
    LitEq(Literal),
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Pattern::*;

        match self {
            Bind(x) => x.fmt(f),
            LitEq(x) => x.fmt(f),
        }
    }
}
