use std::fmt;
use std::rc::Rc;

use derive_more::Deref;
use sappho_unparse::{Stream, Unparse};

use crate::{Coerce, CoercionFailure, Value};

// TODO: Replace `ValRef` with `Value::Object(Rc<...>)`

#[derive(Clone, Debug, PartialEq, Deref)]
#[deref(forward)]
pub struct ValRef(Rc<Value>);

impl ValRef {
    pub fn coerce<T>(&self) -> Result<&T, CoercionFailure>
    where
        T: Coerce,
    {
        T::coerce_from_value(&self.0).ok_or_else(|| CoercionFailure::new::<T>(self))
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
        self.0.as_ref().unparse_into(s)
    }
}
