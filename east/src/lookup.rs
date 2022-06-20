use crate::{AstFxFor, FromFx, GenExpr, Identifier};
use sappho_ast as ast;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Lookup<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub attr: Identifier,
}

impl<FX> From<ast::Lookup<AstFxFor<FX>>> for Lookup<FX>
where
    FX: FromFx,
{
    fn from(lookup: ast::Lookup<AstFxFor<FX>>) -> Self {
        let ast::Lookup { target, attr } = lookup;

        Lookup {
            target: Box::new(GenExpr::from(*target)),
            attr,
        }
    }
}

impl<FX> fmt::Display for Lookup<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.target.fmt(f)?;
        write!(f, ".{}", self.attr)?;
        Ok(())
    }
}
