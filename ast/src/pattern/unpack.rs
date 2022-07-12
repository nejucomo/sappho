use crate::Pattern;
use sappho_fmtutil::{DisplayDepth, FmtResult, Formatter};
use sappho_identmap::{IdentMap, Identifier};
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

impl DisplayDepth for UnpackPattern {
    fn fmt_depth(&self, f: &mut Formatter, depth: usize) -> FmtResult {
        self.0.fmt_depth(f, depth)
    }
}
