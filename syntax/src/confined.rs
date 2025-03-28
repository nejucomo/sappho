use chumsky::Parser as _;
use derive_more::{From, TryInto};
use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
use sappho_identifier::RcId;
use sappho_listform::ListForm;
use sappho_object::Object;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_primval::{Num, PrimVal};
use sappho_unparse::{Stream, Unparse};

use crate::parseparams::ParseParams;
use crate::{BoxWise, FuncDef, ParensExpr, ProcDef, QueryDef, Wise};

use self::Confined::*;

#[derive(Debug, PartialEq, From, TryInto)]
pub enum Confined<FX>
where
    FX: Effect,
{
    Ref(RcId),
    #[from(PrimVal, Num)]
    Prim(PrimVal),
    Parens(ParensExpr<FX>),
    ObjectDef(Object<FuncDef, QueryDef, ProcDef, Wise<FX>>),
    ListExpr(ListForm<Wise<FX>, BoxWise<FX>>),
}

impl<FX> ParsableWith<ParseParams<'_>> for Confined<FX>
where
    FX: Effect + RestrictFrom<ProcEffect>,
{
    fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
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

impl<FX> RestrictFrom<Confined<ProcEffect>> for Confined<FX>
where
    FX: Effect,
{
    fn restrict(src: Confined<ProcEffect>) -> Result<Confined<FX>, Restriction> {
        use Confined::*;

        match src {
            Ref(x) => Ok(Ref(x)),
            Prim(x) => Ok(Prim(x)),
            Parens(x) => ParensExpr::restrict(x).map(Parens),
            ObjectDef(x) => Object::restrict(x).map(ObjectDef),
            ListExpr(x) => ListForm::restrict(x).map(ListExpr),
        }
    }
}
