use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_syntax as syntax;
use sappho_syntax::leftassoc::LeftAssoc;

use crate::transform::TransformInto;
use crate::{Application, Expr, Interaction, Lookup};

impl<FX> From<syntax::Expr<FX>> for Expr<FX>
where
    FX: Effect,
{
    fn from(synex: syntax::Expr<FX>) -> Self {
        synex.transform_into()
    }
}

impl<FX> TransformInto<Expr<FX>> for syntax::Expr<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        use syntax::Expr::*;

        match self {
            Func(x) => x.transform_into().into(),
            Query(x) => x.transform_into().into(),
            Proc(x) => x.transform_into().into(),
            Let(x) => x.transform_into().into(),
            Match(x) => x.transform_into().into(),
            Applications(x) => x.transform_into(),
        }
    }
}

impl<FX> TransformInto<Expr<FX>> for syntax::Applications<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        self.unwrap().transform_into()
    }
}

impl<FX> TransformInto<Expr<FX>> for (Expr<FX>, syntax::Application<FX>)
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        let (target, synapp) = self;
        Application::new(target, synapp.transform_into()).into()
    }
}

impl<FX> TransformInto<Expr<FX>> for syntax::Application<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        self.unwrap().transform_into()
    }
}

impl<FX> TransformInto<Expr<FX>> for syntax::Lookups<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        self.unwrap().transform_into()
    }
}

impl<FX> TransformInto<Expr<FX>> for (Expr<FX>, syntax::Lookup)
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        let (target, attr) = self;
        Lookup::new(target, RcId::from(attr)).into()
    }
}

impl<FX, L, R> TransformInto<Expr<FX>> for LeftAssoc<L, R>
where
    FX: Effect,
    L: TransformInto<Expr<FX>>,
    (Expr<FX>, R): TransformInto<Expr<FX>>,
{
    fn transform_into(self) -> Expr<FX> {
        self.map_left(L::transform_into)
            .fold(|east, syntax| (east, syntax).transform_into())
    }
}

impl<FX> TransformInto<Expr<FX>> for syntax::Interactions<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        self.effects
            .into_iter()
            .fold(self.confined.transform_into(), |target, effect| {
                Interaction::new(effect, target).into()
            })
    }
}

impl<FX> TransformInto<Expr<FX>> for syntax::Confined<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        use syntax::Confined::*;

        match self {
            Ref(x) => x.into(),
            Prim(x) => x.into(),
            ObjectDef(x) => x.transform_into().into(),
            ListExpr(x) => x.transform_into().into(),
            Parens(x) => x.transform_into(),
        }
    }
}

/// Bug: We unwrap Wise and throw away the source link... Fix this impedance mismatch by introducing `WithSource` into syntax at the appropriate places
impl<FX> TransformInto<Expr<FX>> for syntax::ParensExpr<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Expr<FX> {
        self.unwrap().unwrap().unwrap().ignore_source().into()
    }
}
