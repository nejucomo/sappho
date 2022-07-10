mod frame;
mod sref;
mod unbound;

pub use self::frame::{BindFailure, BindFailureReason, Frame};
pub use self::sref::ScopeRef;
pub use self::unbound::{Unbound, UnboundKind};

use crate::ValRef;
use sappho_identmap::IdentRef;

#[derive(Debug)]
pub enum Scope {
    Empty,
    Frame(Frame, ScopeRef),
}

impl Scope {
    pub fn deref(&self, ident: &IdentRef) -> Result<ValRef, Unbound> {
        use crate::UnboundKind::Undeclared;

        self.deref_opt(ident)
            // An `Ok(None)` is an inner value representing no binding declared:
            .and_then(|optval| optval.ok_or_else(|| Undeclared.make(ident)))
    }

    fn deref_opt(&self, ident: &IdentRef) -> Result<Option<ValRef>, Unbound> {
        match self {
            Scope::Empty => Ok(None),
            Scope::Frame(map, lower) => match map.deref(ident) {
                Ok(None) => lower.deref_opt(ident),
                other => other,
            },
        }
    }
}
