use crate::{Coerce, CoercionFailure, Value};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_legible::{IntoNode, Legible, Node};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ValRef(Rc<Value>);

impl ValRef {
    pub fn coerce<T>(&self) -> Result<&T, CoercionFailure>
    where
        T: Coerce,
    {
        T::coerce_from_value(&self.0).ok_or_else(|| CoercionFailure::new::<T>(self))
    }
}

impl Deref for ValRef {
    type Target = Value;

    fn deref(&self) -> &Value {
        self.0.deref()
    }
}

impl Borrow<Value> for ValRef {
    fn borrow(&self) -> &Value {
        self.0.borrow()
    }
}

impl<T> From<T> for ValRef
where
    Value: From<T>,
{
    fn from(v: T) -> Self {
        ValRef(Rc::new(Value::from(v)))
    }
}

impl TryIntoIdentMap<ValRef> for ValRef {
    fn try_into_identmap(&self) -> Option<&IdentMap<ValRef>> {
        self.deref().try_into_identmap()
    }
}

impl fmt::Display for ValRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_legible(f)
    }
}

impl<'a> IntoNode for &'a ValRef {
    fn into_node(self) -> Node {
        self.deref().into_node()
    }
}
