mod clause;

use std::fmt;

pub use self::clause::MatchClause;

/// A `match` expression, ie: `match x { 3 -> 0, y -> y }`.
#[derive(Debug, PartialEq, derive_new::new)]
pub struct MatchExpr<Expr> {
    /// The match target, ie: `x` in `match x { 3 -> 0, y -> y }`.
    pub target: Box<Expr>,

    /// The match clauses, ie: `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
    pub clauses: Vec<MatchClause<Expr>>,
}

impl<X> MatchExpr<X> {
    pub fn transform_into<Y>(self) -> MatchExpr<Y>
    where
        Y: From<X>,
    {
        MatchExpr {
            target: Box::new(Y::from(*self.target)),
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
        }
    }
}

impl<X> fmt::Display for MatchExpr<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "match ")?;
        self.target.fmt(f)?;
        write!(f, " {{ ")?;
        let mut first = true;
        for clause in self.clauses.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            clause.fmt(f)?;
        }
        write!(f, " }}")?;
        Ok(())
    }
}
