use crate::{Coerce, CoercionFailure, Value};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};
use std::borrow::Borrow;
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

impl std::fmt::Display for ValRef {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.unparse(f, 0)
    }
}

impl Unparse for ValRef {
    fn unparse_into(&self, s: &mut Stream) {
        self.deref().unparse(s)
    }
}

// Necessary for value as list form:
impl<'a> Unparse for &'a ValRef {
    fn unparse_into(&self, s: &mut Stream) {
        self.deref().unparse(s)
    }
}
