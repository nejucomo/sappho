use sappho_attrs::Attrs;
use sappho_east::ProcDef;

use crate::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Proc {
    closure: Attrs<Value>,
    qdef: ProcDef,
}
