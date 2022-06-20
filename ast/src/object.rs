use crate::{FuncDef, PureExpr, QueryDef};
use sappho_identmap::IdentMap;
use sappho_object::Object;
use std::fmt;

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Debug, PartialEq)]
pub struct ObjectDef(ObjectInner);
pub type ObjectInner = Object<FuncDef, QueryDef, PureExpr>;

impl std::ops::Deref for ObjectDef {
    type Target = ObjectInner;

    fn deref(&self) -> &ObjectInner {
        &self.0
    }
}

impl ObjectDef {
    pub fn new(func: Option<FuncDef>, query: Option<QueryDef>, attrs: IdentMap<PureExpr>) -> Self {
        ObjectDef(ObjectInner::new(func, query, attrs))
    }

    pub fn unwrap(self) -> ObjectInner {
        self.0
    }
}

impl fmt::Display for ObjectDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(m) = self.0.monolithic() {
            use sappho_object::Monolithic::*;

            match m {
                Func(func) => func.fmt(f),
                Query(query) => query.fmt(f),
                _ => self.0.fmt(f),
            }
        } else {
            self.0.fmt(f)
        }
    }
}
