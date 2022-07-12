use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};

/// A query definition, ie `query $x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct QueryDef<QueryExpr> {
    /// The [QueryExpr] definition, ie the `$x` in `query $x`.
    pub body: Box<QueryExpr>,
}

impl<X> QueryDef<X> {
    pub fn transform_into<Y>(self) -> QueryDef<Y>
    where
        Y: From<X>,
    {
        QueryDef {
            body: Box::new(Y::from(*self.body)),
        }
    }
}

impl<X> DisplayDepth for QueryDef<X>
where
    X: DisplayDepth,
{
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        write!(f, "query ")?;
        self.body.fmt_depth(f, depth)?;
        Ok(())
    }
}
