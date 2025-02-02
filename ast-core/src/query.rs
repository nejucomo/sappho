use sappho_legible::{HeadAndTail, IntoNode, Node};

/// A query definition, ie `query $x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct QueryDef<QueryExpr> {
    /// The `QueryExpr` definition, ie the `$x` in `query $x`.
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

impl<'a, X> IntoNode for &'a QueryDef<X>
where
    &'a X: IntoNode,
{
    fn into_node(self) -> Node {
        HeadAndTail::new("query", " ", &self.body).into_node()
    }
}
