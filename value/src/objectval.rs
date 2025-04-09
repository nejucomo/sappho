use std::fmt;
use std::rc::Rc;

use derive_more::{Deref, From};
use derive_new::new;
use sappho_attrs::errors::Missing;
use sappho_east::{ProcDef, QueryDef};
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_object::Object;
use sappho_primval::PrimVal;

use crate::{CastTo, FuncVal, PseudoType, VResult, Valuable, Value};

#[derive(Clone, Debug, PartialEq, From, Deref)]
#[from(ObjectVal)]
#[deref(forward)]
pub struct ObjectRc(Rc<ObjectVal>);

#[derive(Clone, Debug, PartialEq, Deref, From, new)]
#[new(into)]
pub struct ObjectVal(Object<FuncVal, QueryDef, ProcDef, Value>);

impl PseudoType for ObjectVal {
    fn pseudo_type_name() -> &'static str {
        "object"
    }
}

impl Valuable for ObjectVal {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> VResult<&'s Value, Missing> {
        self.attrs().get(name).map_err(|e| self.wrap_error(e))
    }
}

impl CastTo<PrimVal> for ObjectVal {}

impl CastTo<FuncVal> for ObjectVal {
    fn cast_opt(&self) -> Option<&FuncVal> {
        self.func()
    }
}

impl CastTo<List<Value>> for ObjectVal {}

impl fmt::Display for ObjectVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Fix this to a less lazy, more "native" impl:
        write!(f, "{self:#?}")
    }
}
