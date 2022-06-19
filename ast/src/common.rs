//! Common sub-grammars for all effects

use crate::{Pattern, PureExpr, QueryExpr};
use sappho_identmap::IdentMap;
use sappho_object::Object;
use std::fmt;

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

impl fmt::Display for CommonExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CommonExpr::*;

        match self {
            Func(x) => x.fmt(f),
            Query(x) => x.fmt(f),
            Object(x) => x.fmt(f),
        }
    }
}

impl fmt::Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn ")?;
        self.binding.fmt(f)?;
        write!(f, " -> ")?;
        self.body.fmt(f)?;
        Ok(())
    }
}

impl fmt::Display for QueryDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "query ")?;
        self.body.fmt(f)?;
        Ok(())
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
