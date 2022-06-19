//! Common sub-grammars for all effects

mod func;
mod object;
mod query;

use std::fmt;

pub use self::func::FuncDef;
pub use self::object::ObjectDef;
pub use self::query::QueryDef;

#[derive(Debug, PartialEq)]
pub enum CommonExpr {
    Func(FuncDef),
    Query(QueryDef),
    Object(ObjectDef),
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
