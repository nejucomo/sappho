use sappho_legible::{IntoNode, Joint, Node};

#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct LetClause<Pattern, Expr> {
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: Box<Expr>,
}

impl<P, X> LetClause<P, X> {
    pub fn transform_into<PD, XD>(self) -> LetClause<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        LetClause {
            binding: PD::from(self.binding),
            bindexpr: Box::new(XD::from(*self.bindexpr)),
        }
    }
}

impl<'a, P, X> IntoNode for &'a LetClause<P, X>
where
    &'a P: IntoNode,
    &'a X: IntoNode,
{
    fn into_node(self) -> Node {
        (
            "let ",
            &self.binding,
            " =",
            Joint::from(" "),
            &self.bindexpr,
        )
            .into_node()
    }
}
