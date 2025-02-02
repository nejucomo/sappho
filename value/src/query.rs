use crate::{GenThunk, ScopeRef};
use sappho_ast_core::QueryEffects;
use sappho_ast_reduced::{QueryClause, QueryExpr};
use sappho_legible::{HeadAndTail, IntoNode, Node};

#[derive(Debug)]
pub struct Query {
    body: QueryExpr,
    defscope: ScopeRef,
}

impl Query {
    pub fn new(qc: &QueryClause, defscope: &ScopeRef) -> Self {
        Query {
            body: (*qc.body).clone(),
            defscope: defscope.clone(),
        }
    }

    pub fn as_thunk(&self) -> GenThunk<QueryEffects> {
        GenThunk::new(self.body.clone(), self.defscope.clone())
    }
}

impl<'a> IntoNode for &'a Query {
    fn into_node(self) -> Node {
        HeadAndTail::new("query", " ", &self.body).into_node()
    }
}
