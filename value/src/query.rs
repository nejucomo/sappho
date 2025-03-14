use crate::{GenThunk, ScopeRef};
use sappho_ast_effect::QueryEffect;
use sappho_ast_kernel::QueryDef;
use sappho_ast_red::{AstRed, QueryExpr};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, PartialEq)]
pub struct Query {
    body: QueryExpr,
    defscope: ScopeRef,
}

impl Query {
    pub fn new(qc: &QueryDef<AstRed>, defscope: &ScopeRef) -> Self {
        Query {
            body: (*qc.body).clone(),
            defscope: defscope.clone(),
        }
    }

    pub fn as_thunk(&self) -> GenThunk<QueryEffect> {
        GenThunk::new(self.body.clone(), self.defscope.clone())
    }
}

impl Unparse for Query {
    fn unparse_into(&self, s: &mut Stream) {
        s.write("query ");
        s.write(&self.body);
    }
}
