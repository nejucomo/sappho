use derive_more::From;
use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_primval::PrimVal;
use sappho_syntax as syntax;
use sappho_with_source::WithSource;

use crate::fromhelper::from_unwrap;
use crate::{
    Application, FuncDef, Interaction, Let, ListDef, Lookup, Match, ObjectDef, ProcDef, QueryDef,
};

#[derive(Clone, Debug, PartialEq, From)]
pub enum Expr<FX>
where
    FX: Effect,
{
    #[from]
    Ref(RcId),
    #[from]
    Prim(PrimVal),
    #[from(
        ObjectDef<FX>,
        FuncDef,
        QueryDef,
        ProcDef,
    )]
    ObjectDef(ObjectDef<FX>),
    #[from]
    ListDef(ListDef<FX>),
    #[from]
    Let(Let<FX>),
    #[from]
    Match(Match<FX>),
    #[from]
    Application(Application<FX>),
    #[from]
    Lookup(Lookup<FX>),
    #[from]
    Interaction(Interaction<FX>),
}

impl<FX> From<syntax::Expr<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(synex: syntax::Expr<FX>) -> Self {
        use syntax::Expr::*;
        match synex {
            Func(x) => Self::from(x),
            Query(x) => Self::from(x),
            Proc(x) => Self::from(x),
            Let(x) => Self::from(x),
            Match(x) => Self::from(x),
            Applications(x) => Self::from(x),
        }
    }
}

impl<FX> From<syntax::FuncDef> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::FuncDef) -> Self {
        Self::from(syntax::ObjectDef::from(syn))
    }
}

impl<FX> From<syntax::QueryDef> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::QueryDef) -> Self {
        Self::from(syntax::ObjectDef::from(syn))
    }
}

impl<FX> From<syntax::ProcDef> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::ProcDef) -> Self {
        Self::from(syntax::ObjectDef::from(syn))
    }
}

impl<FX> From<syntax::ObjectDef<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(value: syntax::ObjectDef<FX>) -> Self {
        Self::from(from_unwrap(value))
    }
}

impl<FX> From<syntax::Let<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(value: syntax::Let<FX>) -> Self {
        Self::from(from_unwrap(value))
    }
}

impl<FX> From<syntax::Match<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(value: syntax::Match<FX>) -> Self {
        Self::from(from_unwrap(value))
    }
}

impl<FX> From<syntax::Applications<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::Applications<FX>) -> Self {
        syn.unwrap()
            .map_left(Expr::from)
            .map_rights(Expr::from)
            .fold(|x, arg| Application::new(x, arg).into())
    }
}

impl<FX> From<syntax::Application<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::Application<FX>) -> Self {
        Self::from(syn.unwrap())
    }
}

impl<FX> From<syntax::Lookups<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::Lookups<FX>) -> Self {
        syn.unwrap()
            .map_left(Expr::from)
            .map_rights(RcId::from)
            .fold(|t, attr| Lookup::new(t, attr).into())
    }
}

impl<FX> From<syntax::Interactions<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::Interactions<FX>) -> Self {
        syn.effects
            .into_iter()
            .fold(Self::from(syn.confined), |t, fx| {
                Interaction::new(fx, t).into()
            })
    }
}

impl<FX> From<syntax::Confined<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::Confined<FX>) -> Self {
        use syntax::Confined::*;

        match syn {
            Ref(x) => Expr::Ref(x),
            Prim(x) => Expr::Prim(x),
            ObjectDef(x) => Expr::from(x),
            ListExpr(x) => Expr::from(from_unwrap(x)),
            Parens(x) => Expr::from(x),
        }
    }
}

// This is a weird hack where we throw away the inner source link:
impl<FX> From<syntax::ParensExpr<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(syn: syntax::ParensExpr<FX>) -> Self {
        let syn: syntax::BoxWise<FX> = syn.unwrap();
        let syn: Box<syntax::Wise<FX>> = syn.into();
        let syn: syntax::Wise<FX> = *syn;
        let syn: WithSource<syntax::Expr<FX>> = syn.unwrap();
        let (syn, _): (syntax::Expr<FX>, _) = syn.into();
        Self::from(syn)
    }
}
