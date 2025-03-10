use sappho_object::Object;
use sappho_unparse::Unparse;

use crate::proc::ProcDef;
use crate::query::QueryDef;
use crate::{Expression, FuncDef};

#[derive(Debug, derive_more::From)]
pub struct ObjectDef<X>(Object<FuncDef<X>, QueryDef<X>, ProcDef<X>, X>)
where
    X: Expression;

impl<X> Unparse for ObjectDef<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        s.write(&self.0);
    }
}
