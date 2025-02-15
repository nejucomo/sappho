mod clause;

use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::CoreExpr;

pub use self::clause::MatchClause;

/// A `match` expression, ie: `match x { 3 -> 0, y -> y }`.
#[derive(Clone, Debug, PartialEq, derive_new::new)]
pub struct MatchExpr<FX>
where
    FX: Effect,
{
    /// The match target, ie: `x` in `match x { 3 -> 0, y -> y }`.
    pub target: Box<CoreExpr<FX>>,

    /// The match clauses, ie: `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
    pub clauses: Vec<MatchClause<FX>>,
}

impl<FX> Unparse for MatchExpr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Brackets::Squiggle, Break::OptSpace};

        s.write("match ");
        s.write(&self.target);
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
