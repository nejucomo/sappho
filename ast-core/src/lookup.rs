use sappho_legible::{HeadAndTail, IntoNode, Node};

use crate::Identifier;

/// An attribute lookup expression, ie: `x.foo`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct LookupExpr<Expr> {
    /// The target expression of the lookup, ie `x` in `x.foo`.
    pub target: Box<Expr>,

    /// An attribute name, ie: `foo` in `x.foo`.
    pub attr: Identifier,
}

impl<X> LookupExpr<X> {
    pub fn transform_into<Y>(self) -> LookupExpr<Y>
    where
        Y: From<X>,
    {
        LookupExpr {
            target: Box::new(Y::from(*self.target)),
            attr: self.attr,
        }
    }
}

impl<'a, X> IntoNode for &'a LookupExpr<X>
where
    &'a X: IntoNode,
{
    fn into_node(self) -> Node {
        HeadAndTail::new(&self.target, "", ('.', &self.attr)).into_node()
    }
}
