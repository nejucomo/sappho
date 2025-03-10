use sappho_unparse::{Stream, Unparse};

use crate::{Expression, Kernel, Recursion};

/// An expression which may contain query effects
#[derive(Debug, derive_more::From)]
pub enum QueryExpr<X>
where
    X: Expression,
{
    Kernel(Kernel<X>),
    /// An inquire operation expression: `$x`
    Inquiry(Recursion<X>),
}

impl<X> Unparse for QueryExpr<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        use QueryExpr::*;

        match self {
            Kernel(x) => x.unparse_into(s),
            Inquiry(x) => x.unparse_into(s),
        }
    }
}
