use std::rc::Rc;
use thiserror::Error;

use derive_new::new;
use sappho_east::{FuncDef, PureExpr};

use crate::error::{VResult, ValueResultExt as _};
use crate::{BindError, PseudoType, PseudoTypeError, Scope, Value};

#[derive(Clone, Debug, PartialEq, new)]
pub struct FuncVal {
    #[new(into)]
    closure: Scope,
    #[new(into)]
    fdef: Rc<FuncDef>,
}

#[derive(Debug, Error)]
pub enum ApplicationFailure {
    #[error(transparent)]
    Cast(#[from] PseudoTypeError),
    #[error(transparent)]
    Bind(#[from] BindError),
}

impl FuncVal {
    pub fn apply<F, T>(&self, eval: F, arg: Value) -> VResult<T, ApplicationFailure>
    where
        F: FnOnce(Scope, &PureExpr) -> T,
    {
        let callscope = self
            .closure
            .bind_call_scope(&self.fdef.argpat, arg)
            .convert_inner_err()?;
        Ok(eval(callscope, &self.fdef.body))
    }
}

impl PseudoType for FuncVal {
    fn pseudo_type_name() -> &'static str {
        "fn"
    }
}
