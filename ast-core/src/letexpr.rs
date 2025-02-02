mod clause;

use sappho_legible::{IntoNode, Node, SeparatedSeq};

pub use self::clause::LetClause;

/// A `let` expression for local definitions, ie: `let x = 42; f x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct LetExpr<Pattern, Expr> {
    /// The let clauses:
    pub clauses: Vec<LetClause<Pattern, Expr>>,

    /// The expression to evaluate with the binding in-scope, ie: `f x` in `let x = 42; f x`.
    pub tail: Box<Expr>,
}

impl<P, X> LetExpr<P, X> {
    pub fn transform_into<PD, XD>(self) -> LetExpr<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        LetExpr {
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
            tail: Box::new(XD::from(*self.tail)),
        }
    }
}

impl<'a, P, X> IntoNode for &'a LetExpr<P, X>
where
    &'a P: IntoNode,
    &'a X: IntoNode,
{
    fn into_node(self) -> Node {
        SeparatedSeq::new(
            self.clauses
                .iter()
                .map(|cl| cl.into_node())
                .chain(Some(self.tail.into_node())),
            ";\n",
        )
        .into_node()
    }
}
