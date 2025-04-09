use sappho_list::List;
use sappho_primval::PrimVal;

use crate::{CastTo, FuncVal, ObjectVal, PseudoType, Valuable, Value};

impl PseudoType for List<Value> {
    fn pseudo_type_name() -> &'static str {
        "list"
    }
}

impl Valuable for List<Value> {}

impl CastTo<PrimVal> for List<Value> {}
impl CastTo<ObjectVal> for List<Value> {}
impl CastTo<FuncVal> for List<Value> {}
