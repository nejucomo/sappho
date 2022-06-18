use crate::{Result, ValRef};
use sappho_identmap::IdentMap;
use std::fmt;

pub struct Object(Inner);

type Inner = sappho_object::Object<FuncVal, QueryVal, ValRef>;

impl Object {
    pub fn new(func: Option<FuncVal>, query: Option<QueryVal>, attrs: IdentMap<ValRef>) -> Self {
        Object(Inner::new(func, query, attrs))
    }
}

impl std::ops::Deref for Object {
    type Target = Inner;

    fn deref(&self) -> &Inner {
        &self.0
    }
}

pub type FuncVal = Box<dyn Fn(ValRef) -> Result<ValRef>>;
pub type QueryVal = Box<dyn Fn() -> Result<ValRef>>;

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
