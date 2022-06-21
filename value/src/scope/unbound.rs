use sappho_identmap::{IdentRef, Identifier};
use std::fmt;

#[derive(Debug)]
pub struct Unbound(Identifier);

impl Unbound {
    pub fn new(ident: &IdentRef) -> Self {
        Unbound(ident.to_string())
    }
}

impl fmt::Display for Unbound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unbound: {:?}", &self.0)
    }
}
