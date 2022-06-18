//! Common sub-grammars for all effects

use crate::{Pattern, PureExpr, QueryExpr};
use sappho_identmap::IdentMap;
use sappho_object::Object;

#[derive(Debug, PartialEq)]
pub enum CommonExpr {
    Func(FuncDef),
    Query(QueryDef),
    Object(ObjectDef),
}

#[derive(Debug, PartialEq)]
pub struct FuncDef {
    pub binding: Pattern,
    pub body: Box<PureExpr>,
}

#[derive(Debug, PartialEq)]
pub struct QueryDef {
    pub body: Box<QueryExpr>,
}

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
