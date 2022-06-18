use crate::{List, Object};
use derive_more::From;
use std::fmt;
use std::rc::Rc;

pub type ValRef = Rc<Value>;

#[derive(Debug, From)]
pub enum Value {
    Num(f64),
    List(List),
    Object(Object),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;

        match self {
            Num(x) => x.fmt(f),
            List(x) => x.fmt(f),
            Object(x) => x.fmt(f),
        }
    }
}
