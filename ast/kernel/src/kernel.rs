use crate::{
    ApplicationExpr, AstProvider, EffectExpr, FuncDef, LetExpr, Literal, LookupExpr, MatchExpr,
    ObjectDef, ProcDef, QueryDef,
};
use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_unparse::{Stream, Unparse};

// TODO: Remove Clone/PartialEq impls in favor of derivations w/ XP impl hack

#[derive(Debug, derive_more::From)]
pub enum Kernel<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    #[from(f64)]
    Lit(Literal),
    #[from]
    Ref(RcId),
    #[from]
    Object(ObjectDef<XP, FX>),
    #[from]
    Let(LetExpr<XP, FX>),
    #[from]
    Match(MatchExpr<XP, FX>),
    #[from]
    Application(ApplicationExpr<XP, FX>),
    #[from]
    Lookup(LookupExpr<XP, FX>),
    #[from]
    Effect(EffectExpr<XP, FX>),
}

impl<XP, FX> From<FuncDef<XP>> for Kernel<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: FuncDef<XP>) -> Self {
        Kernel::Object(ObjectDef::from(value))
    }
}

impl<XP, FX> From<QueryDef<XP>> for Kernel<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: QueryDef<XP>) -> Self {
        Kernel::Object(ObjectDef::from(value))
    }
}

impl<XP, FX> From<ProcDef<XP>> for Kernel<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: ProcDef<XP>) -> Self {
        Kernel::Object(ObjectDef::from(value))
    }
}

impl<XP, FX> From<Attrs<XP::Expr<FX>>> for Kernel<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn from(value: Attrs<XP::Expr<FX>>) -> Self {
        Kernel::Object(ObjectDef::from(value))
    }
}

impl<XP, FX> Unparse for Kernel<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn unparse_into(&self, s: &mut Stream) {
        use Kernel::*;

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

impl<XP, FX> Clone for Kernel<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn clone(&self) -> Self {
        use Kernel::*;

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

impl<XP, FX> PartialEq for Kernel<XP, FX>
where
    XP: AstProvider,
    FX: Effect,
{
    fn eq(&self, other: &Self) -> bool {
        use Kernel::*;

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
