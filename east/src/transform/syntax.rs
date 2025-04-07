use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_source::SourceCodeRef;
use sappho_syntax as syntax;
use sappho_with_source::WithSource;

use crate::transform::{TransformInto, TransformWithSource};
use crate::{Application, BoxWise, Expr, Interaction, Lookup};

impl<FX> TransformWithSource<Expr<FX>> for syntax::Applications<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: Option<SourceCodeRef>) -> WithSource<Expr<FX>> {
        self.unwrap()
            .map_left(|l| l.transform_with_source(src.clone()))
            .map_rights(|r| r.transform_with_source(src.clone()))
            .fold(|l, r| WithSource::new(Expr::from(Application::new(l, r)), src.clone()))
    }
}

impl<FX> TransformWithSource<Expr<FX>> for syntax::Application<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: Option<SourceCodeRef>) -> WithSource<Expr<FX>> {
        self.unwrap().transform_with_source(src)
    }
}

impl<FX> TransformWithSource<Expr<FX>> for syntax::Lookups<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: Option<SourceCodeRef>) -> WithSource<Expr<FX>> {
        self.unwrap()
            .map_left(|l| l.transform_with_source(src.clone()))
            .map_rights(RcId::from)
            .fold(|l, r| WithSource::new(Expr::from(Lookup::new(l, r)), src.clone()))
    }
}

impl<FX> TransformWithSource<Expr<FX>> for syntax::Interactions<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: Option<SourceCodeRef>) -> WithSource<Expr<FX>> {
        self.effects.into_iter().fold(
            self.confined.transform_with_source(src.clone()),
            |target, effect| {
                WithSource::new(Expr::from(Interaction::new(effect, target)), src.clone())
            },
        )
    }
}

impl<FX> TransformInto<BoxWise<FX>> for syntax::Confined<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> BoxWise<FX> {
        use syntax::Confined::*;

        match self {
            Ref(x) => BoxWise::from(Expr::Ref(x)),
            Prim(x) => BoxWise::from(x.transform_into()),
            ObjectDef(x) => BoxWise::from(x.transform_into()),
            ListExpr(x) => BoxWise::from(x.transform_into()),
            Parens(x) => x.transform_into(),
        }
    }
}

impl<FX> TransformInto<BoxWise<FX>> for syntax::ParensExpr<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> BoxWise<FX> {
        self.unwrap().transform_into()
    }
}
