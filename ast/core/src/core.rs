use crate::{
    ApplicationExpr, EffectExpr, Identifier, LetExpr, Literal, LookupExpr, MatchExpr, ObjectDef,
};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum CoreExpr<Pattern, PureExpr, QueryExpr, ProcExpr, Expr, FX> {
    Lit(Literal),
    Ref(Identifier),
    Object(ObjectDef<Pattern, PureExpr, QueryExpr, ProcExpr, Expr>),
    Let(LetExpr<Pattern, Expr>),
    Match(MatchExpr<Pattern, Expr>),
    Application(ApplicationExpr<Expr>),
    Lookup(LookupExpr<Expr>),
    Effect(EffectExpr<FX, Expr>),
}

impl<Pat, Pure, Query, Proc, Generic, FX> CoreExpr<Pat, Pure, Query, Proc, Generic, FX> {
    pub fn transform_into<DstPat, DstPure, DstQuery, DstProc, DstGeneric>(
        self,
    ) -> CoreExpr<DstPat, DstPure, DstQuery, DstProc, DstGeneric, FX>
    where
        DstPat: From<Pat>,
        DstPure: From<Pure>,
        DstQuery: From<Query>,
        DstProc: From<Proc>,
        DstGeneric: From<Generic>,
    {
        use CoreExpr::*;

        match self {
            Lit(x) => Lit(x),
            Ref(x) => Ref(x),
            Object(x) => Object(crate::transform_object_def(x)),
            Let(x) => Let(x.transform_into()),
            Match(x) => Match(x.transform_into()),
            Application(x) => Application(x.transform_into()),
            Lookup(x) => Lookup(x.transform_into()),
            Effect(x) => Effect(x.transform_into()),
        }
    }
}

impl<Pat, Pure, Query, Proc, Generic, FX> TryIntoIdentMap<Generic>
    for CoreExpr<Pat, Pure, Query, Proc, Generic, FX>
{
    fn try_into_identmap(&self) -> Option<&IdentMap<Generic>> {
        match self {
            CoreExpr::Object(objdef) => objdef.try_into_identmap(),
            _ => None,
        }
    }
}

impl<Pat, Pure, Query, Proc, Generic, FX> Unparse for CoreExpr<Pat, Pure, Query, Proc, Generic, FX>
where
    Pat: Unparse,
    Pure: Unparse,
    Query: Unparse,
    Proc: Unparse,
    Generic: Unparse,
    FX: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use CoreExpr::*;

        match self {
            Lit(x) => x.unparse_into(s),
            Ref(x) => x.unparse_into(s),
            Object(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            Lookup(x) => x.unparse_into(s),
            Effect(x) => x.unparse_into(s),
        }
    }
}
