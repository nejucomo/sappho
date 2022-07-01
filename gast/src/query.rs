use std::fmt;

/// A query definition, ie `query $x`.
#[derive(Debug, PartialEq, derive_new::new)]
pub struct QueryDef<QueryExpr> {
    /// The [QueryExpr] definition, ie the `$x` in `query $x`.
    pub body: Box<QueryExpr>,
}

impl<X> fmt::Display for QueryDef<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "query ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
