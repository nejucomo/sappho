use chumsky::Parser as _;
use sappho_ast_effect::{Effect, ProcEffect};
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

use crate::Confined::{self, *};
use crate::{ParensExpr, SEParser};

impl ParsableWith<SEParser<'_>> for Confined<ProcEffect> {
    fn make_parser_with(pep: SEParser<'_>) -> impl Parser<Self> {
        RcId::parser()
            .map(Ref)
            .or(PrimVal::parser().map(Prim))
            .or(ParensExpr::parser_with(pep.clone()).map(Parens))
            .or(Object::parser_with(pep.clone()).map(ObjectDef))
            .or(ListForm::parser_with(pep).map(ListExpr))
    }
}

impl<FX> Unparse for Confined<FX>
where
    FX: Effect,
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
