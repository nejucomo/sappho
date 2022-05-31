use crate::{Result, ValRef};
use sappho_ast::Identifier;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub(crate) struct ScopeRef(Rc<Scope>);

impl Default for ScopeRef {
    fn default() -> Self {
        ScopeRef(Rc::new(Scope::Empty))
    }
}

impl ScopeRef {
    pub(crate) fn extend(&self, ident: &str, bindval: ValRef) -> ScopeRef {
        // TODO: Can we remove the ident copy?
        let map = ScopeMap::from([(ident.to_string(), bindval)]);
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

type ScopeMap = BTreeMap<Identifier, ValRef>;

#[derive(Debug)]
pub(crate) enum Scope {
    Empty,
    Frame(ScopeMap, ScopeRef),
}

impl Scope {
    pub(crate) fn deref(&self, ident: &str) -> Result<ValRef> {
        use crate::Error::Unbound;

        self.deref_opt(ident)
            .ok_or_else(|| Unbound(ident.to_string()))
    }

    fn deref_opt(&self, ident: &str) -> Option<ValRef> {
        use Scope::*;

        match self {
            Empty => None,
            Frame(map, lower) => map.get(ident).cloned().or_else(|| lower.deref_opt(ident)),
        }
    }
}
