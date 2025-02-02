mod clause;

use sappho_legible::{BracketSeq, IntoNode, Node};

pub use self::clause::MatchClause;

/// A `match` expression, ie: `match x { 3 -> 0, y -> y }`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct MatchExpr<Pattern, Expr> {
    /// The match target, ie: `x` in `match x { 3 -> 0, y -> y }`.
    pub target: Box<Expr>,

    /// The match clauses, ie: `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
    pub clauses: Vec<MatchClause<Pattern, Expr>>,
}

impl<P, X> MatchExpr<P, X> {
    pub fn transform_into<PD, XD>(self) -> MatchExpr<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        MatchExpr {
            target: Box::new(XD::from(*self.target)),
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
        }
    }
}

impl<'a, P, X> IntoNode for &'a MatchExpr<P, X>
where
    &'a P: IntoNode,
    &'a X: IntoNode,
{
    fn into_node(self) -> Node {
        (
            "match ",
            &self.target,
            " ",
            BracketSeq::new(
                ("{\n", "\n}"),
                ",\n",
                self.clauses.iter().map(|cl| cl.into_node()),
            ),
        )
            .into_node()
    }
}
