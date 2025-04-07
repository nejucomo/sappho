use sappho_effect::Effect;
use sappho_identifier::RcId;
use sappho_source::SourceCodeRef;
use sappho_syntax as syntax;

use crate::transform::{TransformInto, TransformWithSource};
use crate::{Application, Interaction, Lookup, Wise};

impl<FX> TransformWithSource<Wise<FX>> for syntax::Applications<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: &Option<SourceCodeRef>) -> Wise<FX> {
        self.unwrap()
            .map_left(|l| l.transform_with_source(src))
            .map_rights(|r| r.transform_with_source(src))
            .fold(|l, r| Wise::new(Application::new(l, r), src.clone()))
    }
}

impl<FX> TransformWithSource<Wise<FX>> for syntax::Application<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: &Option<SourceCodeRef>) -> Wise<FX> {
        self.unwrap().transform_with_source(src)
    }
}

impl<FX> TransformWithSource<Wise<FX>> for syntax::Lookups<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: &Option<SourceCodeRef>) -> Wise<FX> {
        self.unwrap()
            .map_left(|l| l.transform_with_source(src))
            .map_rights(RcId::from)
            .fold(|l, r| Wise::new(Lookup::new(l, r), src.clone()))
    }
}

impl<FX> TransformWithSource<Wise<FX>> for syntax::Interactions<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: &Option<SourceCodeRef>) -> Wise<FX> {
        self.effects.into_iter().fold(
            self.confined.transform_with_source(src),
            |target, effect| Wise::new(Interaction::new(effect, target), src.clone()),
        )
    }
}

impl<FX> TransformWithSource<Wise<FX>> for syntax::Confined<FX>
where
    FX: Effect,
{
    fn transform_with_source(self, src: &Option<SourceCodeRef>) -> Wise<FX> {
        use syntax::Confined::*;

        match self {
            Ref(x) => Wise::new(x, src.clone()),
            Prim(x) => Wise::new(x, src.clone()),
            ObjectDef(x) => Wise::new(x.transform_into(), src.clone()),
            ListExpr(x) => Wise::new(x.transform_into(), src.clone()),
            Parens(x) => x.transform_into(),
        }
    }
}

impl<FX> TransformInto<Wise<FX>> for syntax::ParensExpr<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Wise<FX> {
        syntax::Wise::transform_into(syntax::BoxWise::unwrap(self.unwrap()))
    }
}
