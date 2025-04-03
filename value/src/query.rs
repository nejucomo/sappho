use sappho_east::QueryDef;

use crate::Scope;

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    closure: Scope,
    qdef: QueryDef,
}
