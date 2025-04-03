use sappho_attrs::Attrs;
use sappho_east::QueryDef;

use crate::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    closure: Attrs<Value>,
    qdef: QueryDef,
}
