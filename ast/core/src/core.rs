use sappho_ast_effect::Effect;
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

use crate::{
    ApplicationExpr, AstProvider, AstTransformInto, CommentedExpr, EffectExpr, Identifier, LetExpr,
    Literal, LookupExpr, MatchExpr, ObjectDef,
};

// TODO: Enable comments for non-expr structures such as match clauses

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

impl<XPS, XPD, FX> AstTransformInto<CoreExpr<XPD, FX>> for CoreExpr<XPS, FX>
where
    XPD: AstProvider,
    XPS: AstProvider,
    XPS::Pattern: AstTransformInto<XPD::Pattern>,
    XPS::Expr<FX>: AstTransformInto<XPD::Expr<FX>>,
    FX: Effect,
{
    fn ast_transform(self) -> CoreExpr<XPD, FX> {
        use CoreExpr::*;

        match self {
            Lit(x) => Lit(x),
            Ref(x) => Ref(x),
            Object(x) => Object(x.ast_transform()),
            Let(x) => Let(x.ast_transform()),
            Match(x) => Match(x.ast_transform()),
            Application(x) => Application(x.ast_transform()),
            Lookup(x) => Lookup(x.ast_transform()),
            Effect(x) => Effect(x.ast_transform()),
        }
    }
}

impl<XP, FX> TryIntoIdentMap<CommentedExpr<XP, FX>> for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn try_into_identmap(&self) -> Option<&IdentMap<CommentedExpr<XP, FX>>> {
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
