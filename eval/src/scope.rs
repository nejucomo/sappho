use crate::{Result, ValRef};
use saplang_ast::Identifier;
use std::collections::HashMap;

type ScopeMap = HashMap<Identifier, ValRef>;

#[derive(Debug)]
pub(crate) enum Scope {
    Empty,
    #[allow(dead_code)]
    Frame(ScopeMap, Box<Scope>),
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
