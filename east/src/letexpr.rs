mod clause;

use crate::{AstFxFor, FromFx, GenExpr};
use sappho_ast as ast;
use std::fmt;

pub use self::clause::LetClause;

#[derive(Debug, PartialEq)]
pub struct LetExpr<Effects> {
    pub clauses: Vec<LetClause<Effects>>,
    pub tail: Box<GenExpr<Effects>>,
}

impl<FX> fmt::Display for LetExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for clause in self.clauses.iter() {
            clause.fmt(f)?;
            write!(f, ";\n")?;
        }
        self.tail.fmt(f)?;
        Ok(())
    }
}

impl<FX> From<ast::LetExpr<AstFxFor<FX>>> for LetExpr<FX>
where
    FX: FromFx,
{
    fn from(le: ast::LetExpr<AstFxFor<FX>>) -> Self {
        LetExpr {
            clauses: le.clauses.into_iter().map(LetClause::from).collect(),
            tail: Box::new(GenExpr::from(*le.tail)),
        }
    }
}
