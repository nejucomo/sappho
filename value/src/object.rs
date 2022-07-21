use crate::{Func, Proc, Query, ValRef};

pub type Object = sappho_object::Object<Func, Query, Proc, ValRef>;
pub type Attrs = sappho_identmap::IdentMap<ValRef>;
