use sappho_legible::{HeadAndTail, IntoNode, Node};

/// Function application, ie `f x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct ApplicationExpr<Expr> {
    /// The target of application, ie `f` in `f x`.
    pub target: Box<Expr>,

    /// The argument of application, ie `x` in `f x`.
    pub argument: Box<Expr>,
}

impl<X> ApplicationExpr<X> {
    pub fn transform_into<Y>(self) -> ApplicationExpr<Y>
    where
        Y: From<X>,
    {
        ApplicationExpr {
            target: Box::new(Y::from(*self.target)),
            argument: Box::new(Y::from(*self.argument)),
        }
    }
}

impl<'a, Expr> IntoNode for &'a ApplicationExpr<Expr>
where
    &'a Expr: IntoNode,
{
    fn into_node(self) -> Node {
        HeadAndTail::new(&self.target, " ", &self.argument).into_node()
    }
}
