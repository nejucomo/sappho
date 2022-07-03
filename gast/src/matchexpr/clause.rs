use std::fmt;

/// A `match` clause, ie `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct MatchClause<Pattern, Expr> {
    /// The binding pattern, ie `3` in `3 -> 0` and the first `y` in `y -> y`.
    pub pattern: Pattern,

    /// The match body expression, ie `0` in `3 -> 0` and the second `y` in `y -> y`.
    pub body: Box<Expr>,
}

impl<P, X> MatchClause<P, X> {
    pub fn transform_into<PD, XD>(self) -> MatchClause<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        MatchClause {
            pattern: PD::from(self.pattern),
            body: Box::new(XD::from(*self.body)),
        }
    }
}

impl<P, X> fmt::Display for MatchClause<P, X>
where
    P: fmt::Display,
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pattern.fmt(f)?;
        write!(f, " -> ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
