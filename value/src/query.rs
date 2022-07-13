use crate::{GenThunk, ScopeRef};
use sappho_east::{QueryClause, QueryEffects, QueryExpr};
use sappho_unparse::{Unparse, Stream};

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

impl Unparse for Query {
    fn unparse_into(&self, s: &mut Stream) {
        write!(f, "query ")?;
        self.body.unparse(f, depth)?;
        Ok(())
    }
}
