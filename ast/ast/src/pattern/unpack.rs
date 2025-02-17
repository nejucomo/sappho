use crate::Pattern;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_unparse::{Stream, Unparse};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct UnpackPattern(Attrs<Pattern>);

impl UnpackPattern {
    pub fn unwrap(self) -> Attrs<Pattern> {
        self.0
    }
}

impl FromIterator<(RcId, Pattern)> for UnpackPattern
where
    Attrs<Pattern>: FromIterator<(RcId, Pattern)>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (RcId, Pattern)>,
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
