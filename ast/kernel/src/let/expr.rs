use sappho_unparse::{Stream, Unparse};

use crate::r#let::LetClause;
use crate::{Expression, Recursion};

#[derive(Debug, derive_new::new)]
pub struct Let<X>
where
    X: Expression,
{
    clauses: Vec<LetClause<X>>,
    tail: Recursion<X>,
}

impl<X> Unparse for Let<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Brackets::Parens, Break::Mandatory};

        let unparse_clauses = |s: &mut Stream| {
            for (ix, clause) in self.clauses.iter().enumerate() {
                if s.depth() > 0 || ix > 0 {
                    s.write(&Mandatory);
                }
                s.write(clause);
                s.write(";");
            }
            s.write(&Mandatory);
            s.write(&self.tail);
        };

        if s.depth() == 0 {
            unparse_clauses(s);
        } else {
            s.bracketed(Parens, unparse_clauses);
        }
    }
}
