use crate::ValRef;
use std::fmt;

#[derive(Debug)]
pub struct CoercionFailure(ValRef, &'static str);

impl CoercionFailure {
    pub fn new<T>(v: &ValRef) -> Self {
        CoercionFailure(v.clone(), std::any::type_name::<T>())
    }
}

impl fmt::Display for CoercionFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unbound: {:?}", &self.0)
    }
}
