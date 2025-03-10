use sappho_unparse::{Stream, Unparse};

use crate::{QueryX, Recursion};

/// A [ProcX] is an expression with proc effects
#[derive(Debug, derive_more::From)]
pub enum ProcX<U>
where
    U: Unparse,
{
    QueryX(QueryX<U>),
    /// An invoke operation expression: `!x`
    Invocation(Recursion<U>),
}

impl<U> Unparse for ProcX<U>
where
    U: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use ProcX::*;

        match self {
            QueryX(x) => x.unparse_into(s),
            Invocation(x) => x.unparse_into(s),
        }
    }
}
