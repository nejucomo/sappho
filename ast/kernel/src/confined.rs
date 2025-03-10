use sappho_identifier::RcId;
use sappho_primval::PrimVal;

use crate::Root;

#[derive(Debug, derive_more::From)]
pub enum Confined<R>
where
    R: Root,
{
    Prim(PrimVal),
    Ref(RcId),
    Parens(Box<R>),
}
