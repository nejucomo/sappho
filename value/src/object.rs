use crate::{Func, Query, ValRef};

pub type Object = sappho_object::Object<Func, Query, ValRef>;
pub type Attrs = sappho_identmap::IdentMap<ValRef>;
