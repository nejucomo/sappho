use std::fmt;
use std::rc::Rc;

use derive_more::{Deref, From};
use derive_new::new;
use sappho_attrs::errors::Missing;
use sappho_attrs::Attrs;
use sappho_east::{FuncDef, ProcDef, QueryDef};
use sappho_identifier::RcId;
use sappho_object::Object;

use crate::{AsError, Scoped, VResult, Valuable, Value};

#[derive(Clone, Debug, PartialEq, Deref, From, new)]
#[new(into)]
pub struct ObjectVal(Object<FuncVal, QueryVal, ProcVal, Value>);

pub type FuncVal = Scoped<Rc<FuncDef>>;
pub type QueryVal = Scoped<Rc<QueryDef>>;
pub type ProcVal = Scoped<Rc<ProcDef>>;

impl ObjectVal {
    pub fn new_from_parts(
        f: Option<FuncVal>,
        q: Option<QueryVal>,
        p: Option<ProcVal>,
        attrs: Attrs<Value>,
    ) -> Self {
        ObjectVal(Object::new(f, q, p, attrs))
    }
}

impl Valuable for ObjectVal {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> VResult<&'s Value, Missing> {
        self.attrs().get(name).map_err(|e| self.wrap_error(e))
    }

    fn apply(&self, argument: Value) -> VResult<Value, AsError> {
        if let Some(funcdef) = self.func() {
            // BUG: We need cyclic dependency on `eval`
            let _ = (argument, funcdef);
            todo!("figure out how to manage eval<->value<->ast interdependency.")
        } else {
            Err(self.wrap_error(AsError("fn")))
        }
    }
}

impl fmt::Display for ObjectVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Fix this to a less lazy, more "native" impl:
        write!(f, "{self:#?}")
    }
}
