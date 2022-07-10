mod frame;
mod sref;
mod unbound;

pub use self::frame::{BindFailure, BindFailureReason, Frame};
pub use self::sref::ScopeRef;
pub use self::unbound::Unbound;

use crate::ValRef;
use sappho_identmap::IdentRef;

#[derive(Debug)]
pub enum Scope {
    Empty,
    Frame(Frame, ScopeRef),
}

impl Scope {
    pub fn deref(&self, ident: &IdentRef) -> Result<ValRef, Unbound> {
        self.deref_opt(ident).ok_or_else(|| Unbound::new(ident))
    }

    fn deref_opt(&self, ident: &IdentRef) -> Option<ValRef> {
        match self {
            Scope::Empty => None,
            Scope::Frame(map, lower) => map.get(ident).cloned().or_else(|| lower.deref_opt(ident)),
        }
    }
}
