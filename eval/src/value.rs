use crate::{Coerce, Func, List, Object};
use derive_more::From;
use std::fmt;

#[derive(Debug, From)]
pub enum Value {
    Num(f64),
    List(List),
    Object(Object),
}
use Value::*;

impl Coerce for f64 {
    fn coerce_from_value(v: &Value) -> Option<&f64> {
        match v {
            Num(x) => Some(x),
            _ => None,
        }
    }
}

impl Coerce for Object {
    fn coerce_from_value(v: &Value) -> Option<&Object> {
        match v {
            Object(x) => Some(x),
            _ => None,
        }
    }
}

impl Coerce for Func {
    fn coerce_from_value(v: &Value) -> Option<&Func> {
        Object::coerce_from_value(v).and_then(|obj| obj.func())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Num(x) => x.fmt(f),
            List(x) => x.fmt(f),
            Object(x) => x.fmt(f),
        }
    }
}
