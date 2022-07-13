mod clause;

use sappho_unparse::{Unparse, Stream};

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

impl<P, X> Unparse for LetExpr<P, X>
where
    P: Unparse,
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Unparse, Stream};

        let (indented, cdepth) = if depth == 0 {
            (false, 0)
        } else {
            (true, depth + 1)
        };

        if indented {
            writeln!(f, "(")?;
        }

        for clause in self.clauses.iter() {
            indent(f, cdepth)?;
            clause.unparse(f, cdepth)?;
            writeln!(f, ";")?;
        }
        indent(f, cdepth)?;
        self.tail.unparse(f, cdepth)?;

        if indented {
            writeln!(f)?;
            indent(f, depth)?;
            write!(f, ")")?;
        }
        Ok(())
    }
}
