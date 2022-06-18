use crate::{Pattern, PureExpr, QueryExpr};
use sappho_ast as ast;
use sappho_identmap::IdentMap;
use sappho_object::Object;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct ObjectDef(ObjectInner);
pub type ObjectInner = Object<FuncClause, QueryClause, PureExpr>;

impl std::ops::Deref for ObjectDef {
    type Target = ObjectInner;

    fn deref(&self) -> &ObjectInner {
        &self.0
    }
}

impl From<ast::ObjectDef> for ObjectDef {
    fn from(od: ast::ObjectDef) -> ObjectDef {
        ObjectDef(
            od.unwrap()
                .transform(FuncClause::from, QueryClause::from, PureExpr::from),
        )
    }
}

impl From<ast::FuncDef> for ObjectDef {
    fn from(d: ast::FuncDef) -> ObjectDef {
        ObjectDef(ObjectInner::new(
            Some(FuncClause::from(d)),
            None,
            IdentMap::default(),
        ))
    }
}

impl From<ast::QueryDef> for ObjectDef {
    fn from(d: ast::QueryDef) -> ObjectDef {
        ObjectDef(ObjectInner::new(
            None,
            Some(QueryClause::from(d)),
            IdentMap::default(),
        ))
    }
}

impl From<ast::CommonExpr> for ObjectDef {
    fn from(x: ast::CommonExpr) -> Self {
        use ast::CommonExpr::*;

        match x {
            Func(x) => ObjectDef::from(x),
            Query(x) => ObjectDef::from(x),
            Object(x) => ObjectDef::from(x),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FuncClause {
    pub binding: Pattern,
    pub body: Rc<PureExpr>,
}

impl From<ast::FuncDef> for FuncClause {
    fn from(fd: ast::FuncDef) -> FuncClause {
        FuncClause {
            binding: fd.binding,
            body: Rc::new(PureExpr::from(*fd.body)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct QueryClause {
    pub body: Rc<QueryExpr>,
}

impl From<ast::QueryDef> for QueryClause {
    fn from(qd: ast::QueryDef) -> QueryClause {
        QueryClause {
            body: Rc::new(QueryExpr::from(*qd.body)),
        }
    }
}
