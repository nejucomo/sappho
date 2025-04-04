use std::fmt;

use derive_more::From;
use sappho_attrs::errors::Missing;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_primval::{Num, PrimVal};

use crate::{AsError, ObjectRc, ObjectVal, VResult, Valuable};

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

    fn as_list(&self) -> VResult<&List<Value>, AsError> {
        match self {
            VPrim(x) => x.as_list(),
            VObj(x) => x.as_list(),
            VList(x) => x.as_list(),
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
