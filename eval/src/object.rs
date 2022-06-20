use crate::{Func, Query, ValRef};
use sappho_identmap::IdentMap;
use std::fmt;

pub struct Object(Inner);

type Inner = sappho_object::Object<Func, Query, ValRef>;

impl Object {
    pub fn new(func: Option<Func>, query: Option<Query>, attrs: IdentMap<ValRef>) -> Self {
        Object(Inner::new(func, query, attrs))
    }
}

impl std::ops::Deref for Object {
    type Target = Inner;

    fn deref(&self) -> &Inner {
        &self.0
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        sappho_object::Object::new(
            self.func().map(|_| "fn … -> …"),
            self.query().map(|_| "query …"),
            self.attrs().map_value_refs(|_| "…"),
        )
        .fmt(f)
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
