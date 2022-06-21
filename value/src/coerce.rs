mod error;

use crate::{Attrs, Func, Object, Value};

pub use self::error::CoercionFailure;

pub trait Coerce {
    fn coerce_from_value(v: &Value) -> Option<&Self>;
}

impl Coerce for f64 {
    fn coerce_from_value(v: &Value) -> Option<&f64> {
        match v {
            Value::Num(x) => Some(x),
            _ => None,
        }
    }
}

impl Coerce for Object {
    fn coerce_from_value(v: &Value) -> Option<&Object> {
        match v {
            Value::Object(x) => Some(x),
            _ => None,
        }
    }
}

impl Coerce for Func {
    fn coerce_from_value(v: &Value) -> Option<&Func> {
        Object::coerce_from_value(v).and_then(|obj| obj.func())
    }
}

impl Coerce for Attrs {
    fn coerce_from_value(v: &Value) -> Option<&Attrs> {
        Object::coerce_from_value(v).map(|obj| obj.attrs())
    }
}
