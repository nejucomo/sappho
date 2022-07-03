mod clause;

use std::fmt;

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

impl<P, X> fmt::Display for MatchExpr<P, X>
where
    P: fmt::Display,
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use sappho_fmtutil::fmt_comma_sep;

        write!(f, "match ")?;
        self.target.fmt(f)?;
        write!(f, " {{ ")?;
        fmt_comma_sep(&self.clauses, f)?;
        write!(f, " }}")?;
        Ok(())
    }
}
