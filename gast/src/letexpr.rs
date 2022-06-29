mod clause;

use std::fmt;

pub use self::clause::LetClause;

/// A `let` expression for local definitions, ie: `let x = 42; f x`.
#[derive(Debug, PartialEq)]
pub struct LetExpr<Expr> {
    /// The let clauses:
    pub clauses: Vec<LetClause<Expr>>,

    /// The expression to evaluate with the binding in-scope, ie: `f x` in `let x = 42; f x`.
    pub tail: Box<Expr>,
}

impl<X> fmt::Display for LetExpr<X>
where
    X: fmt::Display,
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
