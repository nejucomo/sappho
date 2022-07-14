use crate::{Object, ValRef};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_more::From)]
pub enum Value {
    Num(f64),
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

impl Unparse for Value {
    fn unparse_into(&self, s: &mut Stream) {
        use Value::*;

        match self {
            Num(x) => s.write(&x.to_string()),
            Object(x) => {
                if let Some(list) = x.try_into_identmap().and_then(|m| m.as_list_form()) {
                    s.write(&list)
                } else {
                    s.write(x)
                }
            }
        }
    }
}
