mod clause;

use crate::{AstFxFor, FromFx, GenExpr};
use sappho_ast as ast;
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

impl<FX> From<ast::MatchExpr<AstFxFor<FX>>> for MatchExpr<FX>
where
    FX: FromFx,
{
    fn from(ame: ast::MatchExpr<AstFxFor<FX>>) -> Self {
        MatchExpr {
            target: Box::new(GenExpr::from(*ame.target)),
            clauses: ame.clauses.into_iter().map(MatchClause::from).collect(),
        }
    }
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
