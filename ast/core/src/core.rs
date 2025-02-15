use crate::{
    ApplicationExpr, EffectExpr, FuncDef, Identifier, LetExpr, ListExpr, Literal, LookupExpr,
    MatchExpr, ObjectDef, ProcDef, QueryDef,
};
use sappho_ast_effect::Effect;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum CoreExpr<FX>
where
    FX: Effect,
{
    Lit(Literal),
    Ref(Identifier),
    Object(ObjectDef<FX>),
    Func(FuncDef),
    Query(QueryDef),
    Proc(ProcDef),
    List(ListExpr<FX>),
    Let(LetExpr<FX>),
    Match(MatchExpr<FX>),
    Application(ApplicationExpr<FX>),
    Lookup(LookupExpr<FX>),
    Effect(EffectExpr<FX>),
}

impl<FX> TryIntoIdentMap<CoreExpr<FX>> for CoreExpr<FX>
where
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<CoreExpr<FX>>> {
        match self {
            CoreExpr::Object(objdef) => objdef.try_into_identmap(),
            _ => None,
        }
    }
}

impl<FX> Unparse for CoreExpr<FX>
where
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        use CoreExpr::*;

        match self {
            Lit(x) => x.unparse_into(s),
            Ref(x) => x.unparse_into(s),
            Object(x) => x.unparse_into(s),
            Func(x) => x.unparse_into(s),
            Query(x) => x.unparse_into(s),
            Proc(x) => x.unparse_into(s),
            List(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            Lookup(x) => x.unparse_into(s),
            Effect(x) => x.unparse_into(s),
        }
    }
}
