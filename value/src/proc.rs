use sappho_east::ProcDef;

use crate::Scope;

#[derive(Clone, Debug, PartialEq)]
pub struct Proc {
    closure: Scope,
    qdef: ProcDef,
}
