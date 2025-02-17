use crate::Pattern;
use sappho_attrs::{Attrs, Identifier};
use sappho_unparse::{Stream, Unparse};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct UnpackPattern(Attrs<Pattern>);

impl UnpackPattern {
    pub fn unwrap(self) -> Attrs<Pattern> {
        self.0
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
