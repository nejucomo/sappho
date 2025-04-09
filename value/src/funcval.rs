use std::rc::Rc;

use derive_new::new;
use sappho_east::FuncDef;

use crate::{PseudoType, Scope};

#[derive(Clone, Debug, PartialEq, new)]
pub struct FuncVal {
    closure: Scope,
    fdef: Rc<FuncDef>,
}

impl PseudoType for FuncVal {
    fn pseudo_type_name() -> &'static str {
        "fn"
    }
}
