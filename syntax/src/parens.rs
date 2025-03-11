use chumsky::Parser as _;
use sappho_identifier::RcId;
use sappho_parsable::{Parsable, Parser};
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

use crate::Parens;

impl<FX> Parsable for Parens<FX>
where
    FX: Parsable,
{
    fn parser() -> impl Parser<Self> {
        xxx
    }
}

impl<FX> Unparse for Parens<FX>
where
    FX: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Ref(x) => x.unparse_into(s),
            Prim(x) => x.unparse_into(s),
            Parens(x) => x.unparse_into(s),
            ObjectDef(x) => x.unparse_into(s),
            ListExpr(x) => x.unparse_into(s),
        }
    }
}
