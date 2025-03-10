use sappho_object::Object;
use sappho_unparse::Unparse;

use crate::QueryDef;

// TODO:
type FuncDef<T> = QueryDef<T>;
type ProcDef<T> = QueryDef<T>;

#[derive(Debug, derive_more::From)]
pub struct ObjectDef<R>(Object<FuncDef<R>, QueryDef<R>, ProcDef<R>, R>)
where
    R: Unparse;

impl<R> Unparse for ObjectDef<R>
where
    R: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        s.write(&self.0);
    }
}
