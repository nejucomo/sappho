use crate::{Coerce, CoercionFailure, Value};
use sappho_unparse::{Stream, Unparse};
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

// TODO: Replace `ValRef` with `Value::Object(Rc<...>)`

#[derive(Debug)]
pub struct ValRef(Rc<Value>);

impl ValRef {
    pub fn coerce<T>(&self) -> Result<&T, CoercionFailure>
    where
        T: Coerce,
    {
        T::coerce_from_value(&self.0).ok_or_else(|| CoercionFailure::new::<T>(self))
    }
}

impl Clone for ValRef {
    fn clone(&self) -> Self {
        ValRef(self.0.clone())
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

impl fmt::Display for ValRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unparse().fmt(f)
    }
}

impl Unparse for ValRef {
    fn unparse_into(&self, s: &mut Stream) {
        self.deref().unparse_into(s)
    }
}

// Necessary for value as list form:
impl Unparse for &ValRef {
    fn unparse_into(&self, s: &mut Stream) {
        (*self).unparse_into(s)
    }
}
