use crate::Pattern;
use std::fmt;

/// A `match` clause, ie `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct MatchClause<Expr> {
    /// The binding pattern, ie `3` in `3 -> 0` and the first `y` in `y -> y`.
    pub pattern: Pattern,

    /// The match body expression, ie `0` in `3 -> 0` and the second `y` in `y -> y`.
    pub body: Box<Expr>,
}

impl<X> MatchClause<X> {
    pub fn transform_into<Y>(self) -> MatchClause<Y>
    where
        Y: From<X>,
    {
        MatchClause {
            pattern: self.pattern,
            body: Box::new(Y::from(*self.body)),
        }
    }
}

impl<X> fmt::Display for MatchClause<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pattern.fmt(f)?;
        write!(f, " -> ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
