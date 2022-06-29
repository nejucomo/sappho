use crate::{FuncDef, QueryDef};
use sappho_identmap::IdentMap;
use sappho_object::Object;
use std::fmt;

/// An object definition expression, ie `{ x: 42, y: 7, fn x -> x }`.
#[derive(Debug, PartialEq)]
pub struct ObjectDef<PureExpr, QueryExpr>(ObjectInner<PureExpr, QueryExpr>);
pub type ObjectInner<PureExpr, QueryExpr> =
    Object<FuncDef<PureExpr>, QueryDef<QueryExpr>, PureExpr>;

impl<P, Q> std::ops::Deref for ObjectDef<P, Q> {
    type Target = ObjectInner<P, Q>;

    fn deref(&self) -> &ObjectInner<P, Q> {
        &self.0
    }
}

impl<P, Q> ObjectDef<P, Q> {
    pub fn new(func: Option<FuncDef<P>>, query: Option<QueryDef<Q>>, attrs: IdentMap<P>) -> Self {
        ObjectDef(ObjectInner::new(func, query, attrs))
    }

    pub fn unwrap(self) -> ObjectInner<P, Q> {
        self.0
    }
}

impl<P, Q> fmt::Display for ObjectDef<P, Q>
where
    P: fmt::Display,
    Q: fmt::Display,
{
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
