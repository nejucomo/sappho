use crate::Pattern;
use sappho_ast as ast;
use sappho_identmap::{IdentMap, Identifier};
use std::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq, derive_more::From)]
pub struct UnpackPattern(IdentMap<Pattern>);

impl From<ast::UnpackPattern> for UnpackPattern {
    fn from(aup: ast::UnpackPattern) -> Self {
        UnpackPattern::from(aup.unwrap().into_map_values(Pattern::from))
    }
}

impl From<UnpackPattern> for ast::UnpackPattern {
    fn from(eup: UnpackPattern) -> Self {
        ast::UnpackPattern::from(eup.0.into_map_values(ast::Pattern::from))
    }
}

impl FromIterator<(Identifier, Pattern)> for UnpackPattern
where
    IdentMap<Pattern>: FromIterator<(Identifier, Pattern)>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Identifier, Pattern)>,
    {
        UnpackPattern(IdentMap::from_iter(iter))
    }
}

impl Deref for UnpackPattern {
    type Target = IdentMap<Pattern>;

    fn deref(&self) -> &IdentMap<Pattern> {
        &self.0
    }
}

impl fmt::Display for UnpackPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
