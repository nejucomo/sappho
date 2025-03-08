use sappho_syntax_unparse::{Stream, Unparse};

/// A literal value, such as `3.1415`.
#[derive(Copy, Clone, Debug, PartialEq, derive_more::From)]
pub enum Literal {
    /// A literal number value, such as `42`.
    Num(f64),
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal::Num(f64::from(i))
    }
}

impl Unparse for Literal {
    fn unparse_into(&self, s: &mut Stream) {
        use Literal::*;

        match self {
            Num(x) => s.write(&x.to_string()),
        }
    }
}
