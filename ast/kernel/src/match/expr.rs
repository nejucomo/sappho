use sappho_unparse::{Stream, Unparse};

use crate::r#match::MatchClause;
use crate::{Expression, Recursion};

#[derive(Debug, derive_new::new)]
pub struct Match<X>
where
    X: Expression,
{
    candidate: Recursion<X>,
    clauses: Vec<MatchClause<X>>,
}

impl<X> Unparse for Match<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Brackets::Squiggle, Break::OptSpace};

        s.write("match ");
        s.write(&self.candidate);
        s.write(" ");
        s.bracketed(Squiggle, |subs| {
            for clause in &self.clauses {
                subs.write(&OptSpace);
                subs.write(clause);
                subs.write(",");
            }
        });
    }
}
