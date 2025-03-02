use crate::{Func, Proc, Query, ValRef};

pub type Object = sappho_object::Object<Func, Query, Proc, ValRef>;
pub type AttrVals = sappho_attrs::Attrs<ValRef>;
