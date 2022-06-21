use crate::{Attrs, Scope, ValRef};
use sappho_identmap::IdentRef;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ScopeRef(Rc<Scope>);

impl Default for ScopeRef {
    fn default() -> Self {
        ScopeRef(Rc::new(Scope::Empty))
    }
}

impl ScopeRef {
    pub fn extend(&self, ident: &IdentRef, bindval: ValRef) -> ScopeRef {
        // TODO: Can we remove the ident copy?
        let map = Attrs::from([(ident.to_string(), bindval)]);
        let frame = Scope::Frame(map, self.clone());
        ScopeRef(Rc::new(frame))
    }
}

impl Deref for ScopeRef {
    type Target = Scope;

    fn deref(&self) -> &Scope {
        self.0.deref()
    }
}
