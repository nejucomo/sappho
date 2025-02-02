use crate::{Object, ValRef};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_legible::{IntoNode, Node};

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

impl<'a> IntoNode for &'a Value {
    fn into_node(self) -> Node {
        use Value::*;

        match self {
            Num(x) => x.to_string().into_node(),
            Object(x) => {
                if let Some(list) = x.try_into_identmap().and_then(|m| m.as_list_form()) {
                    list.into_node()
                } else {
                    x.into_node()
                }
            }
        }
    }
}
