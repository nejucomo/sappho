mod func;
mod query;

use crate::PureExpr;
use sappho_ast as ast;
use sappho_identmap::IdentMap;
use sappho_object::Object;
use std::fmt;

pub use self::func::FuncClause;
pub use self::query::QueryClause;

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

impl fmt::Display for ObjectDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
