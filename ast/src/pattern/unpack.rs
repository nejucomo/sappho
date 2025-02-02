use crate::Pattern;
use sappho_identmap::{IdentMap, Identifier};
use sappho_legible::{IntoNode, Node};
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

impl IntoNode for &UnpackPattern {
    fn into_node(self) -> Node {
        self.0.into_node()
    }
}
