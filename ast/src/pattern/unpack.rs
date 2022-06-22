use crate::Pattern;
use sappho_identmap::IdentMap;
use std::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub struct UnpackPattern(IdentMap<Pattern>);

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
