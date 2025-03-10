use sappho_unparse::{Stream, Unparse};

use crate::{QueryExpr, Recursion};

/// An expression which may contain proc or query effects
#[derive(Debug, derive_more::From)]
pub enum ProcExpr<U>
where
    U: Unparse,
{
    QueryExpr(QueryExpr<U>),
    /// An invoke operation expression: `!x`
    Invocation(Recursion<U>),
}

impl<U> Unparse for ProcExpr<U>
where
    U: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use ProcExpr::*;

        match self {
            QueryExpr(x) => x.unparse_into(s),
            Invocation(x) => x.unparse_into(s),
        }
    }
}
