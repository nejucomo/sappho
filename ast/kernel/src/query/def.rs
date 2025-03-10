use sappho_unparse::{Stream, Unparse};

use crate::query::QueryExpr;
use crate::{Expression, Recursion};

/// # Todo
///
/// Change grammar to require parens around definition, ie: query ( $x ), or perhaps have a confined expression?
#[derive(Debug, derive_more::From)]
#[from(QueryExpr<X>)]
pub struct QueryDef<X>(Recursion<QueryExpr<X>>)
where
    X: Expression;

impl<X> Unparse for QueryDef<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("query ");
        s.write(&self.0);
    }
}
