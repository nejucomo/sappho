mod clause;

use crate::GenExpr;
use std::fmt;

pub use self::clause::LetClause;

/// A `let` expression for local definitions, ie: `let x = 42; f x`.
#[derive(Debug, PartialEq)]
pub struct LetExpr<Effects> {
    /// The let clauses:
    pub clauses: Vec<LetClause<Effects>>,

    /// The expression to evaluate with the binding in-scope, ie: `f x` in `let x = 42; f x`.
    pub tail: Box<GenExpr<Effects>>,
}

impl<FX> fmt::Display for LetExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for clause in self.clauses.iter() {
            clause.fmt(f)?;
            writeln!(f, ";")?;
        }
        self.tail.fmt(f)?;
        Ok(())
    }
}
