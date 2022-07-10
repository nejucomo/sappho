mod sref;
mod unbound;

pub use self::sref::{BindFailure, BindFailureReason, ScopeRef};
pub use self::unbound::Unbound;

use crate::{Attrs, ValRef};
use sappho_identmap::IdentRef;

#[derive(Debug)]
pub enum Scope {
    Empty,
    Frame(Attrs, ScopeRef),
}

impl Scope {
    pub fn deref(&self, ident: &IdentRef) -> Result<ValRef, Unbound> {
        self.deref_opt(ident).ok_or_else(|| Unbound::new(ident))
    }

    fn deref_opt(&self, ident: &IdentRef) -> Option<ValRef> {
        use Scope::*;

        match self {
            Empty => None,
            Frame(map, lower) => map.get(ident).cloned().or_else(|| lower.deref_opt(ident)),
        }
    }
}
