use crate::{
    ApplicationExpr, AstProvider, EffectExpr, FuncDef, LetExpr, LookupExpr, MatchExpr, ObjectDef,
    ProcDef, QueryDef,
};
use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_syntax_universal::Unx;
use sappho_syntax_unparse::{Stream, Unparse};

// TODO: Remove Clone/PartialEq impls in favor of derivations w/ XP impl hack

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    Unx(Unx),
    Object(ObjectDef<XP, FX>),
    Let(LetExpr<XP, FX>),
    Match(MatchExpr<XP, FX>),
    Application(ApplicationExpr<XP, FX>),
    Lookup(LookupExpr<XP, FX>),
    Effect(EffectExpr<XP, FX>),
}

impl<XP, FX> From<FuncDef<XP>> for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: FuncDef<XP>) -> Self {
        CoreExpr::Object(ObjectDef::from(value))
    }
}

impl<XP, FX> From<QueryDef<XP>> for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: QueryDef<XP>) -> Self {
        CoreExpr::Object(ObjectDef::from(value))
    }
}

impl<XP, FX> From<ProcDef<XP>> for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: ProcDef<XP>) -> Self {
        CoreExpr::Object(ObjectDef::from(value))
    }
}

impl<XP, FX> From<Attrs<XP::Expr<FX>>> for CoreExpr<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: Attrs<XP::Expr<FX>>) -> Self {
        CoreExpr::Object(ObjectDef::from(value))
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
            Unx(x) => x.unparse_into(s),
            Object(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            Lookup(x) => x.unparse_into(s),
            Effect(x) => x.unparse_into(s),
        }
    }
}
