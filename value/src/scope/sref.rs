use crate::{BindFailure, Frame, Scope, ValRef};
use sappho_east::Pattern;
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
    pub fn bind(&self, pattern: &Pattern, val: &ValRef) -> Result<ScopeRef, BindFailure> {
        let frame = Frame::from_pattern_binding(pattern, val)?;
        Ok(self.extend(frame))
    }

    fn extend(&self, frame: Frame) -> ScopeRef {
        let scope = Scope::Frame(frame, self.clone());
        ScopeRef(Rc::new(scope))
    }
}

impl Deref for ScopeRef {
    type Target = Scope;

    fn deref(&self) -> &Scope {
        self.0.deref()
    }
}
