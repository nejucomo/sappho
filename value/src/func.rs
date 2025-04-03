use sappho_east::FuncDef;

use crate::Scope;

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    closure: Scope,
    fdef: FuncDef,
}
