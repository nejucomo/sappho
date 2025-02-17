use sappho_attrs::{IdentRef, Identifier};
use std::fmt;

#[derive(Debug)]
pub struct Unbound {
    ident: Identifier,
    kind: UnboundKind,
}

#[derive(Copy, Clone, Debug)]
pub enum UnboundKind {
    Undeclared,
    Unfulfilled,
}

impl UnboundKind {
    pub fn make(self, ident: &IdentRef) -> Unbound {
        Unbound {
            ident: ident.to_string(),
            kind: self,
        }
    }
}

impl fmt::Display for Unbound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unbound {:?} is {}", &self.ident, self.kind)
    }
}

impl fmt::Display for UnboundKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnboundKind::*;

        match self {
            Undeclared => "not in scope",
            Unfulfilled => "a not-yet-defined forward reference",
        }
        .fmt(f)
    }
}
