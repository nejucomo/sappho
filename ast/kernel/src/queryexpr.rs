use sappho_unparse::{Stream, Unparse};

use crate::{Kernel, Recursion};

/// An expression which may contain query effects
#[derive(Debug, derive_more::From)]
pub enum QueryExpr<U>
where
    U: Unparse,
{
    Kernel(Kernel<U>),
    /// An inquire operation expression: `$x`
    Inquiry(Recursion<U>),
}

impl<U> Unparse for QueryExpr<U>
where
    U: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use QueryExpr::*;

        match self {
            Kernel(x) => x.unparse_into(s),
            Inquiry(x) => x.unparse_into(s),
        }
    }
}
