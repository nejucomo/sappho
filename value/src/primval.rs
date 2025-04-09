use sappho_list::List;
use sappho_primval::PrimVal;

use crate::{CastTo, FuncVal, ObjectVal, PseudoType, Valuable, Value};

impl PseudoType for PrimVal {
    fn pseudo_type_name() -> &'static str {
        "prim-val"
    }
}

impl Valuable for PrimVal {}

impl CastTo<ObjectVal> for PrimVal {}
impl CastTo<FuncVal> for PrimVal {}
impl CastTo<List<Value>> for PrimVal {}
