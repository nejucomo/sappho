use std::fmt;
use std::ops::Deref as _;
use std::rc::Rc;

use derive_more::{Deref, From};
use derive_new::new;
use sappho_attrs::errors::Missing;
use sappho_east::{FuncDef, ProcDef, QueryDef};
use sappho_identifier::RcId;
use sappho_object::Object;

use crate::{FuncRef, Scope, VResult, Valuable, Value};

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

impl<'a> TryFrom<&'a ObjectRc> for FuncRef<'a> {
    type Error = &'a ObjectRc;

    fn try_from(value: &'a ObjectRc) -> Result<Self, Self::Error> {
        value.deref().try_into().map_err(|_| value)
    }
}

impl<'a> TryFrom<&'a ObjectVal> for FuncRef<'a> {
    type Error = &'a ObjectVal;

    fn try_from(value: &'a ObjectVal) -> Result<Self, Self::Error> {
        value
            .obj
            .func()
            .map(|fdef| FuncRef::new(&value.closure, fdef))
            .ok_or(value)
    }
}

impl fmt::Display for ObjectRc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Fix this to a less lazy, more "native" impl:
        write!(f, "{self:#?}")
    }
}
