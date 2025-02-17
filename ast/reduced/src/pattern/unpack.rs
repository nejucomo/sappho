use crate::Pattern;
use sappho_ast as ast;
use sappho_attrs::{Attrs, Identifier};
use sappho_unparse::{Stream, Unparse};
use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq, derive_more::From)]
pub struct UnpackPattern(Attrs<Pattern>);

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
    Attrs<Pattern>: FromIterator<(Identifier, Pattern)>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Identifier, Pattern)>,
    {
        UnpackPattern(Attrs::from_iter(iter))
    }
}

impl Deref for UnpackPattern {
    type Target = Attrs<Pattern>;

    fn deref(&self) -> &Attrs<Pattern> {
        &self.0
    }
}

impl Unparse for UnpackPattern {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
