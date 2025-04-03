use sappho_attrs::Attrs;
use sappho_east::FuncDef;

use crate::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    closure: Attrs<Value>,
    fdef: FuncDef,
}
