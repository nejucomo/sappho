use sappho_ast_effect::QueryEffect;
use sappho_unparse::{Stream, Unparse};

use crate::CoreExpr;

/// A query definition, ie `query $x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct QueryDef {
    /// The `QueryExpr` definition, ie the `$x` in `query $x`.
    pub body: Box<CoreExpr<QueryEffect>>,
}

impl Unparse for QueryDef {
    fn unparse_into(&self, s: &mut Stream) {
        s.write("query ");
        s.write(&self.body);
    }
}
