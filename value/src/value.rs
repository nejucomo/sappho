use crate::{List, Object, ValRef};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use std::fmt;

#[derive(Debug, derive_more::From)]
pub enum Value {
    Num(f64),
    List(List),
    Object(Box<Object>),
}

impl TryIntoIdentMap<ValRef> for Value {
    fn try_into_identmap(&self) -> Option<&IdentMap<ValRef>> {
        match self {
            Value::Object(obj) => obj.try_into_identmap(),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;

        match self {
            Num(x) => x.fmt(f),
            List(x) => x.fmt(f),
            Object(x) => {
                if let Some(list) = x.try_into_identmap().and_then(|m| m.as_list_form()) {
                    list.fmt(f)
                } else {
                    x.fmt(f)
                }
            }
        }
    }
}
