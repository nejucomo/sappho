use crate::{GenThunk, ScopeRef};
use sappho_east::{QueryClause, QueryEffects, QueryExpr};
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
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

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "query ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
