use crate::ScopeRef;
use sappho_east::{QueryClause, QueryExpr};
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

    // FIXME: introduce generic `Thunk` for eval.
    pub fn peek(&self) -> (&QueryExpr, &ScopeRef) {
        (&self.body, &self.defscope)
    }
}
