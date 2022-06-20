use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Literal {
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
