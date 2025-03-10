use sappho_unparse::{Stream, Unparse};

use crate::{Kernel, Recursion};

/// A [QueryX] is an expression with query effects
#[derive(Debug, derive_more::From)]
pub enum QueryX<U>
where
    U: Unparse,
{
    Kernel(Kernel<U>),
    /// An inquire operation expression: `$x`
    Inquiry(Recursion<U>),
}

impl<U> Unparse for QueryX<U>
where
    U: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use QueryX::*;

        match self {
            Kernel(x) => x.unparse_into(s),
            Inquiry(x) => x.unparse_into(s),
        }
    }
}
