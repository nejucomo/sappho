use std::fmt;
use std::rc::Rc;

use derive_more::{Deref, From};
use derive_new::new;
use sappho_attrs::errors::Missing;
use sappho_east::{FuncDef, ProcDef, QueryDef};
use sappho_identifier::RcId;
use sappho_object::Object;

use crate::{AsError, Scope, VResult, Valuable, Value};

#[derive(Clone, Debug, PartialEq, From, Deref)]
#[from(ObjectVal)]
#[deref(forward)]
pub struct ObjectRc(Rc<ObjectVal>);

#[derive(Clone, Debug, PartialEq, Deref, From, new)]
pub struct ObjectVal {
    closure: Scope,
    #[deref]
    #[new(into)]
    obj: Object<FuncDef, QueryDef, ProcDef, Value>,
}

impl Valuable for ObjectRc {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> VResult<&'s Value, Missing> {
        self.attrs().get(name).map_err(|e| self.wrap_error(e))
    }

    fn apply(&self, argument: Value) -> VResult<Value, AsError> {
        if let Some(funcdef) = self.obj.func() {
            // BUG: We need cyclic dependency on `eval`
            let _ = (argument, funcdef);
            todo!("figure out how to manage eval<->value<->ast interdependency.")
        } else {
            Err(self.wrap_error(AsError("fn")))
        }
    }
}

impl fmt::Display for ObjectRc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Fix this to a less lazy, more "native" impl:
        write!(f, "{self:#?}")
    }
}
