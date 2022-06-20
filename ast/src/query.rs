use crate::QueryExpr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct QueryDef {
    pub body: Box<QueryExpr>,
}

impl fmt::Display for QueryDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "query ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
