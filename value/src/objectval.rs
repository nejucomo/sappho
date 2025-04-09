use std::fmt;
use std::rc::Rc;

use derive_more::{Deref, From};
use derive_new::new;
use sappho_attrs::errors::Missing;
use sappho_east::{FuncDef, ProcDef, QueryDef};
use sappho_identifier::RcId;
use sappho_object::Object;

use crate::{Scope, VResult, Valuable, Value};

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
}

impl fmt::Display for ObjectRc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Fix this to a less lazy, more "native" impl:
        write!(f, "{self:#?}")
    }
}
