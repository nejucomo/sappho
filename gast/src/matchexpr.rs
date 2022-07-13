mod clause;

use sappho_unparse::{Stream, Unparse};

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

impl<P, X> Unparse for MatchExpr<P, X>
where
    P: Unparse,
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write_str("match ");
        self.target.unparse(s);
        s.write_str(" {");
        let mut subs = Stream::new();
        for clause in &self.clauses {
            clause.unparse(&mut subs);
            subs.write_str_break(",", true);
        }
        s.add_substream(subs);
        s.write_str("}");
    }
}
