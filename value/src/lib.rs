use derive_more::From;
use sappho_primval::{Num, PrimVal};

#[derive(Clone, Debug, PartialEq, From)]
pub enum Value {
    #[from(PrimVal, Num)]
    PrimVal(PrimVal),
}
