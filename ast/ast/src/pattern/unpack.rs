use crate::Pattern;
use sappho_identmap::{IdentMap, Identifier};
use sappho_unparse::{Stream, Unparse};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct UnpackPattern(IdentMap<Pattern>);

impl UnpackPattern {
    pub fn unwrap(self) -> IdentMap<Pattern> {
        self.0
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

impl Unparse for UnpackPattern {
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
