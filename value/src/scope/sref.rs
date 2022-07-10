mod bind;
mod bindfailure;

use self::bind::bind_attrs;
use crate::{Attrs, Scope, ValRef};
use sappho_east::Pattern;
use std::ops::Deref;
use std::rc::Rc;

pub use self::bindfailure::{BindFailure, BindFailureReason};

#[derive(Clone, Debug)]
pub struct ScopeRef(Rc<Scope>);

impl Default for ScopeRef {
    fn default() -> Self {
        ScopeRef(Rc::new(Scope::Empty))
    }
}

impl ScopeRef {
    pub fn bind(&self, pattern: &Pattern, val: &ValRef) -> Result<ScopeRef, BindFailure> {
        let attrs = bind_attrs(pattern, val)?;
        Ok(self.extend(attrs))
    }

    fn extend(&self, attrs: Attrs) -> ScopeRef {
        let frame = Scope::Frame(attrs, self.clone());
        ScopeRef(Rc::new(frame))
    }
}

impl Deref for ScopeRef {
    type Target = Scope;

    fn deref(&self) -> &Scope {
        self.0.deref()
    }
}
