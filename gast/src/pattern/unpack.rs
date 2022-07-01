use crate::Pattern;
use sappho_identmap::{IdentMap, Identifier};
use std::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct UnpackPattern(IdentMap<Pattern>);

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
