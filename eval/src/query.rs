use crate::eval::Eval;
use crate::scope::ScopeRef;
use crate::{Result, ValRef};
use sappho_east::{QueryClause, QueryExpr};
use std::rc::Rc;

pub struct Query {
    body: Rc<QueryExpr>,
    defscope: ScopeRef,
}

impl Query {
    pub(crate) fn new(qc: &QueryClause, defscope: &ScopeRef) -> Self {
        Query {
            body: qc.body.clone(),
            defscope: defscope.clone(),
        }
    }

    pub fn query(&self) -> Result<ValRef> {
        self.body.eval(&self.defscope)
    }
}
