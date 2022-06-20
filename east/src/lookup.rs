use crate::{AstFxFor, FromFx, GenExpr, Identifier};
use sappho_ast as ast;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LookupExpr<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub attr: Identifier,
}

impl<FX> From<ast::LookupExpr<AstFxFor<FX>>> for LookupExpr<FX>
where
    FX: FromFx,
{
    fn from(lookup: ast::LookupExpr<AstFxFor<FX>>) -> Self {
        let ast::LookupExpr { target, attr } = lookup;

        LookupExpr {
            target: Box::new(GenExpr::from(*target)),
            attr,
        }
    }
}

impl<FX> fmt::Display for LookupExpr<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.target.fmt(f)?;
        write!(f, ".{}", self.attr)?;
        Ok(())
    }
}
