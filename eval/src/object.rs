use crate::{Result, ValRef};
use sappho_identmap::IdentMap;
use std::fmt;

pub struct Object {
    pub func: Option<Box<dyn Fn(ValRef) -> Result<ValRef>>>,
    pub attrs: IdentMap<ValRef>,
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "<object{} attrs: {:?}>",
            if self.func.is_some() { " fn" } else { "" },
            self.attrs.keys().collect::<Vec<&String>>(),
        )
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
