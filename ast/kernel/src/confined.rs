use sappho_identifier::RcId;
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

use crate::Root;

use self::Confined::*;

#[derive(Debug, derive_more::From)]
pub enum Confined<R>
where
    R: Root,
{
    Prim(PrimVal),
    Ref(RcId),
    Parens(Box<R>),
}

impl<R> Unparse for Confined<R>
where
    R: Root,
{
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Prim(x) => x.unparse_into(s),
            Ref(x) => x.unparse_into(s),
            Parens(x) => x.unparse_into(s),
        }
    }
}
