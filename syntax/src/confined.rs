use chumsky::Parser as _;
use sappho_identifier::RcId;
use sappho_parsable::{Parsable, Parser};
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

use crate::Confined::{self, *};

impl<FX> Parsable for Confined<FX>
where
    FX: Parsable,
{
    fn parser() -> impl Parser<Self> {
        RcId::parser()
            .map(Ref)
            .or(PrimVal::parser().map(Prim))
            .or(chumsky::primitive::todo())
    }
}

impl<FX> Unparse for Confined<FX>
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
