use crate::{GenThunk, ScopeRef};
use sappho_east::{QueryClause, QueryEffects, QueryExpr};
use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};

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

impl DisplayDepth for Query {
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        write!(f, "query ")?;
        self.body.fmt_depth(f, depth)?;
        Ok(())
    }
}
