use sappho_object::Object;

use crate::Wise;

#[derive(Debug)]
pub struct ObjectDef<FX>(Object<FuncDef, QueryDef, ProcDef, Wise<FX>>);
