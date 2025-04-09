use derive_new::new;
use sappho_east::FuncDef;

use crate::Scope;

#[derive(Clone, Debug, PartialEq, new)]
pub struct FuncRef<'a> {
    pub closure: &'a Scope,
    pub fdef: &'a FuncDef,
}
