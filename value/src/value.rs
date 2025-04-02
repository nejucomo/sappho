use std::fmt;

use derive_more::From;
use sappho_attrs::AttrsError;
use sappho_identifier::RcId;
use sappho_primval::{Num, PrimVal};

use crate::Valuable;

use self::Value::*;

#[derive(Clone, Debug, PartialEq, From)]
pub enum Value {
    #[from(PrimVal, Num)]
    Prim(PrimVal),
    #[from]
    List(sappho_list::List<Value>),
}

impl Valuable for Value {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> Result<&'s Value, AttrsError> {
        match self {
            Prim(x) => x.attr_lookup(name),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prim(x) => (*x).fmt(f),
        }
    }
}
