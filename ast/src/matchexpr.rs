mod clause;

use crate::GenExpr;
use std::fmt;

pub use self::clause::MatchClause;

/// A `match` expression, ie: `match x { 3 -> 0, y -> y }`.
#[derive(Debug, PartialEq)]
pub struct MatchExpr<Effects> {
    /// The match target, ie: `x` in `match x { 3 -> 0, y -> y }`.
    pub target: Box<GenExpr<Effects>>,

    /// The match clauses, ie: `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
    pub clauses: Vec<MatchClause<Effects>>,
}

impl<FX> fmt::Display for MatchExpr<FX>
where
    FX: fmt::Display,
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
