use sappho_unparse::{Stream, Unparse};

use crate::query::QueryExpr;
use crate::{Expression, Recursion};

/// An expression which may contain proc or query effects
#[derive(Debug, derive_more::From)]
pub enum ProcExpr<X>
where
    X: Expression,
{
    QueryExpr(QueryExpr<X>),
    /// An invoke operation expression: `!x`
    Invocation(Recursion<X>),
}

impl<X> Unparse for ProcExpr<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        use ProcExpr::*;

        match self {
            QueryExpr(x) => x.unparse_into(s),
            Invocation(x) => x.unparse_into(s),
        }
    }
}
