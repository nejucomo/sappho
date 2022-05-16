use crate::{Pattern, PureExpr, QueryExpr};
use saplang_ast as ast;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct ObjectDef {
    pub query: Option<QueryClause>,
    pub func: Option<FuncClause>,
}

impl From<ast::ObjectDef> for ObjectDef {
    fn from(od: ast::ObjectDef) -> ObjectDef {
        ObjectDef {
            query: od.query.map(|d| QueryClause::from(d)),
            func: od.func.map(|d| FuncClause::from(d)),
        }
    }
}

impl From<ast::FuncDef> for ObjectDef {
    fn from(d: ast::FuncDef) -> ObjectDef {
        ObjectDef {
            query: None,
            func: Some(FuncClause::from(d)),
        }
    }
}

impl From<ast::QueryDef> for ObjectDef {
    fn from(d: ast::QueryDef) -> ObjectDef {
        ObjectDef {
            query: Some(QueryClause::from(d)),
            func: None,
        }
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
