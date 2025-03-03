mod failure;

use crate::{AttrVals, Func, Object, Query, Value};

pub use self::failure::CoercionFailure;

pub trait Coerce {
    fn sappho_type_name() -> &'static str;
    fn coerce_from_value(v: &Value) -> Option<&Self>;
}

impl Coerce for f64 {
    fn sappho_type_name() -> &'static str {
        "num"
    }

    fn coerce_from_value(v: &Value) -> Option<&f64> {
        match v {
            Value::Num(x) => Some(x),
            _ => None,
        }
    }
}

impl Coerce for Object {
    fn sappho_type_name() -> &'static str {
        "object"
    }

    fn coerce_from_value(v: &Value) -> Option<&Object> {
        match v {
            Value::Object(x) => Some(x),
            _ => None,
        }
    }
}

impl Coerce for Func {
    fn sappho_type_name() -> &'static str {
        "fn"
    }

    fn coerce_from_value(v: &Value) -> Option<&Func> {
        Object::coerce_from_value(v).and_then(|obj| obj.func())
    }
}

impl Coerce for Query {
    fn sappho_type_name() -> &'static str {
        "query"
    }

    fn coerce_from_value(v: &Value) -> Option<&Query> {
        Object::coerce_from_value(v).and_then(|obj| obj.query())
    }
}

impl Coerce for AttrVals {
    fn sappho_type_name() -> &'static str {
        "attributes"
    }

    fn coerce_from_value(v: &Value) -> Option<&AttrVals> {
        Object::coerce_from_value(v).map(|obj| obj.attrs())
    }
}
