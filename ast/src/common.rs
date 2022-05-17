//! Common sub-grammars for all effects

use crate::{Pattern, PureExpr, QueryExpr};

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
pub struct ObjectDef {
    pub query: Option<QueryDef>,
    pub func: Option<FuncDef>,
}
