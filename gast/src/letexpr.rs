mod clause;

use std::fmt;

pub use self::clause::LetClause;

/// A `let` expression for local definitions, ie: `let x = 42; f x`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct LetExpr<Pattern, Expr> {
    /// The let clauses:
    pub clauses: Vec<LetClause<Pattern, Expr>>,

    /// The expression to evaluate with the binding in-scope, ie: `f x` in `let x = 42; f x`.
    pub tail: Box<Expr>,
}

impl<P, X> LetExpr<P, X> {
    pub fn transform_into<PD, XD>(self) -> LetExpr<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        LetExpr {
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
            tail: Box::new(XD::from(*self.tail)),
        }
    }
}

impl<P, X> fmt::Display for LetExpr<P, X>
where
    P: fmt::Display,
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
