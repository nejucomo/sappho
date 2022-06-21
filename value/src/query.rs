use crate::{GenThunk, ScopeRef};
use sappho_east::{QueryClause, QueryEffects, QueryExpr};
use std::rc::Rc;

pub struct Query {
    body: Rc<QueryExpr>,
    defscope: ScopeRef,
}

impl Query {
    pub fn new(qc: &QueryClause, defscope: &ScopeRef) -> Self {
        Query {
            body: qc.body.clone(),
            defscope: defscope.clone(),
        }
    }

    pub fn as_thunk(&self) -> GenThunk<QueryEffects> {
        GenThunk::new(&self.body, self.defscope.clone())
    }
}
