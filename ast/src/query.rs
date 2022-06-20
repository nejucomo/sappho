use crate::QueryExpr;
use std::fmt;

/// A query definition, ie `query $x`.
#[derive(Debug, PartialEq)]
pub struct QueryDef {
    /// The [QueryExpr] definition, ie the `$x` in `query $x`.
    pub body: Box<QueryExpr>,
}

impl fmt::Display for QueryDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "query ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
