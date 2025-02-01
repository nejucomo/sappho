use sappho_legible::{IntoNode, Node};

#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct EffectExpr<FX, Expr> {
    pub effect: FX,
    pub expr: Box<Expr>,
}

impl<FX, X> EffectExpr<FX, X> {
    pub fn transform_into<Y>(self) -> EffectExpr<FX, Y>
    where
        Y: From<X>,
    {
        EffectExpr {
            effect: self.effect,
            expr: Box::new(Y::from(*self.expr)),
        }
    }
}

impl<'a, FX, X> IntoNode for &'a EffectExpr<FX, X>
where
    &'a FX: IntoNode,
    &'a X: IntoNode,
{
    fn into_node(self) -> Node {
        (&self.effect, &self.expr).into_node()
    }
}
