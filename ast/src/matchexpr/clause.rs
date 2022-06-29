use crate::GenExpr;
use sappho_gast::Pattern;
use std::fmt;

/// A `match` clause, ie `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
#[derive(Debug, PartialEq)]
pub struct MatchClause<Effects> {
    /// The binding pattern, ie `3` in `3 -> 0` and the first `y` in `y -> y`.
    pub pattern: Pattern,

    /// The match body expression, ie `0` in `3 -> 0` and the second `y` in `y -> y`.
    pub body: Box<GenExpr<Effects>>,
}

impl<FX> fmt::Display for MatchClause<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pattern.fmt(f)?;
        write!(f, " -> ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}
