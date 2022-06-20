use std::fmt;

/// A literal value, such as `3.1415`.
#[derive(Debug, PartialEq)]
pub enum Literal {
    /// A literal number value, such as `42`.
    Num(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Literal::*;

        match self {
            Num(x) => x.fmt(f),
        }
    }
}
