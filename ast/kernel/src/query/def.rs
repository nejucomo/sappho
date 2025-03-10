use sappho_unparse::{Stream, Unparse};

use crate::QueryExpr;

/// # Todo
///
/// Change grammar to require parens around definition, ie: query ( $x ), or perhaps have a confined expression?
#[derive(Debug, derive_more::From)]
#[from(QueryExpr<R>)]
pub struct QueryDef<R>(Box<QueryExpr<R>>)
where
    R: Unparse;

impl<R> Unparse for QueryDef<R>
where
    R: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("query ");
        s.write(&self.0);
    }
}
