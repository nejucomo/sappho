use sappho_unparse::{Unparse, Stream};

/// A literal value, such as `3.1415`.
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Literal {
    /// A literal number value, such as `42`.
    Num(f64),
}

impl Unparse for Literal {
    fn unparse(&self) -> Stream {
        use std::fmt::Display;
        use Literal::*;

        match self {
            Num(x) => x.fmt(f),
        }
    }
}
