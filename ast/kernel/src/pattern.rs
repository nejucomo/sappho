use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_primval::PrimVal;
use sappho_unparse::Unparse;

use crate::Expression;

use self::Pattern::*;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Pattern<X>
where
    X: Expression,
{
    Bind(RcId),
    LitEq(PrimVal),
    Unpack(Attrs<X::Pattern>),
}

impl<X> Unparse for Pattern<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        match self {
            Bind(x) => x.unparse_into(s),
            LitEq(x) => x.unparse_into(s),
            Unpack(x) => x.unparse_into(s),
        }
    }
}
