use sappho_unparse::{Stream, Unparse};

use crate::tryaslist::TryAsList;
use crate::Object;

#[derive(Debug, derive_more::From)]
pub enum Value {
    Num(f64),
    Object(Box<Object>),
}

impl Unparse for Value {
    fn unparse_into(&self, s: &mut Stream) {
        use Value::*;

        match self {
            Num(x) => s.write(&x.to_string()),
            Object(x) => {
                if let Some(list) = x.try_as_list() {
                    s.write(&list)
                } else {
                    s.write(x)
                }
            }
        }
    }
}
