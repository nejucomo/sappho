use std::rc::Rc;

use derive_new::new;
use sappho_east::{FuncDef, PureExpr};

use crate::{BindError, PseudoType, Scope, VResult, Value};

#[derive(Clone, Debug, PartialEq, new)]
pub struct FuncVal {
    closure: Scope,
    fdef: Rc<FuncDef>,
}

impl FuncVal {
    pub fn apply<F, T>(&self, eval: F, arg: Value) -> VResult<T, BindError>
    where
        F: FnOnce(Scope, &PureExpr) -> VResult<T, BindError>,
    {
        let callscope = self.closure.bind_call_scope(&self.fdef.argpat, arg)?;
        eval(callscope, &self.fdef.body)
    }
}

impl PseudoType for FuncVal {
    fn pseudo_type_name() -> &'static str {
        "fn"
    }
}
