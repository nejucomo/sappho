use std::fmt;

use derive_more::From;
use sappho_attrs::errors::Missing;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_primval::{Num, PrimVal};

use crate::{CastTo, FuncVal, ObjectRc, ObjectVal, VResult, Valuable};

use self::Value::*;

#[derive(Clone, Debug, PartialEq, From)]
pub enum Value {
    #[from(PrimVal, Num)]
    VPrim(PrimVal),
    #[from(ObjectRc, ObjectVal)]
    VObj(ObjectRc),
    #[from]
    VList(List<Value>),
}

impl Valuable for Value {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> VResult<&'s Value, Missing> {
        match self {
            VPrim(x) => x.attr_lookup(name),
            VObj(x) => x.attr_lookup(name),
            VList(x) => x.attr_lookup(name),
        }
    }
}

impl CastTo<PrimVal> for Value {
    fn cast_opt(&self) -> Option<&PrimVal> {
        match self {
            VPrim(x) => Some(x),
            _ => None,
        }
    }
}

impl CastTo<ObjectVal> for Value {
    fn cast_opt(&self) -> Option<&ObjectVal> {
        match self {
            VObj(x) => Some(x),
            _ => None,
        }
    }
}

impl CastTo<FuncVal> for Value {
    fn cast_opt(&self) -> Option<&FuncVal> {
        self.cast_opt().and_then(|obj: &ObjectVal| obj.cast_opt())
    }
}

impl CastTo<List<Value>> for Value {
    fn cast_opt(&self) -> Option<&List<Value>> {
        match self {
            VList(x) => Some(x),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VPrim(x) => (*x).fmt(f),
            VObj(x) => x.fmt(f),
            VList(x) => x.fmt(f),
        }
    }
}
