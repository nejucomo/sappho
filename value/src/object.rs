use std::rc::Rc;

use sappho_object::Object;

use crate::Func;

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectRef(Rc<Object<Func, (), (), ()>>);
