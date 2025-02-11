mod clause;

use sappho_ast_effect::Effect;
use sappho_unparse::{Stream, Unparse};

use crate::AstProvider;

pub use self::clause::MatchClause;

/// A `match` expression, ie: `match x { 3 -> 0, y -> y }`.
#[derive(Debug, derive_new::new)]
pub struct MatchExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    /// The match target, ie: `x` in `match x { 3 -> 0, y -> y }`.
    pub target: Box<XP::Expr<FX>>,

    /// The match clauses, ie: `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
    pub clauses: Vec<MatchClause<XP, FX>>,
}

impl<XP, FX> MatchExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> MatchExpr<XPD, FX>
    where
        XPD: AstProvider,
        XPD::Pattern: From<XP::Pattern>,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
    {
        MatchExpr {
            target: Box::new(XPD::Expr::from(*self.target)),
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
        }
    }
}

impl<XP, FX> Unparse for MatchExpr<XP, FX>
where
    XP: AstProvider,
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

impl<XP, FX> Clone for MatchExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        MatchExpr::new(self.target.clone(), self.clauses.clone())
    }
}

impl<XP, FX> PartialEq for MatchExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target && self.clauses == other.clauses
    }
}
