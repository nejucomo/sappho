use crate::{
    ApplicationExpr, AstProvider, EffectExpr, Identifier, LetExpr, Literal, LookupExpr, MatchExpr,
    ObjectDef,
};
use sappho_ast_effect::{Effect, ProcEffect, PureEffect, QueryEffect};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_more::From)]
pub enum CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    Lit(Literal),
    Ref(Identifier),
    Object(ObjectDef<XP, FX>),
    Let(LetExpr<XP, FX>),
    Match(MatchExpr<XP, FX>),
    Application(ApplicationExpr<XP, FX>),
    Lookup(LookupExpr<XP, FX>),
    Effect(EffectExpr<XP, FX>),
}

impl<XP, FX> CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    pub fn transform_into<XPD>(self) -> CoreExpr<XPD, FX>
    where
        XPD: AstProvider,
        XPD::Pattern: From<XP::Pattern>,
        XPD::Expr<FX>: From<XP::Expr<FX>>,
        XPD::Expr<PureEffect>: From<XP::Expr<PureEffect>>,
        XPD::Expr<QueryEffect>: From<XP::Expr<QueryEffect>>,
        XPD::Expr<ProcEffect>: From<XP::Expr<ProcEffect>>,
    {
        use CoreExpr::*;

        match self {
            Lit(x) => Lit(x),
            Ref(x) => Ref(x),
            Object(x) => Object(x.transform_into()),
            Let(x) => Let(x.transform_into()),
            Match(x) => Match(x.transform_into()),
            Application(x) => Application(x.transform_into()),
            Lookup(x) => Lookup(x.transform_into()),
            Effect(x) => Effect(x.transform_into()),
        }
    }
}

impl<XP, FX> TryIntoIdentMap<XP::Expr<FX>> for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<XP::Expr<FX>>> {
        match self {
            CoreExpr::Object(objdef) => objdef.try_into_identmap(),
            _ => None,
        }
    }
}

impl<XP, FX> Unparse for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
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

impl<XP, FX> Clone for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        use CoreExpr::*;

        match self {
            Lit(x) => Lit(*x),
            Ref(x) => Ref(x.clone()),
            Object(x) => Object(x.clone()),
            Let(x) => Let(x.clone()),
            Match(x) => Match(x.clone()),
            Application(x) => Application(x.clone()),
            Lookup(x) => Lookup(x.clone()),
            Effect(x) => Effect(x.clone()),
        }
    }
}

impl<XP, FX> PartialEq for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        use CoreExpr::*;

        match (self, other) {
            (Lit(l), Lit(r)) => l == r,
            (Ref(l), Ref(r)) => l == r,
            (Object(l), Object(r)) => l == r,
            (Let(l), Let(r)) => l == r,
            (Match(l), Match(r)) => l == r,
            (Application(l), Application(r)) => l == r,
            (Lookup(l), Lookup(r)) => l == r,
            (Effect(l), Effect(r)) => l == r,
            _ => false,
        }
    }
}
