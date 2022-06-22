use crate::{Coerce, ValRef};
use std::fmt;

#[derive(Debug)]
pub struct CoercionFailure(ValRef, &'static str);

impl CoercionFailure {
    pub fn new<T>(v: &ValRef) -> Self
    where
        T: Coerce,
    {
        CoercionFailure(v.clone(), T::sappho_type_name())
    }
}

impl fmt::Display for CoercionFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let CoercionFailure(v, typename) = self;
        write!(f, "cannot coerce {} into {}", v, typename)
    }
}
