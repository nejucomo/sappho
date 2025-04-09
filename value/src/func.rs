use derive_new::new;
use sappho_east::FuncDef;

use crate::Scope;

#[derive(Clone, Debug, PartialEq, new)]
pub struct Func {
    closure: Scope,
    fdef: FuncDef,
}
