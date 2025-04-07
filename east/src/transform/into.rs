use sappho_effect::Effect;
use sappho_kast::{BoxWise, KastProvider, Wise};
use sappho_with_source::WithSource;

use crate::transform::TransformWithSource;

pub(crate) trait TransformInto<T> {
    fn transform_into(self) -> T;
}

impl<S, T> TransformInto<Vec<T>> for Vec<S>
where
    S: TransformInto<T>,
{
    fn transform_into(self) -> Vec<T> {
        self.into_iter().map(S::transform_into).collect()
    }
}

impl<S, T> TransformInto<WithSource<T>> for WithSource<S>
where
    S: TransformWithSource<T>,
{
    fn transform_into(self) -> WithSource<T> {
        let (parsed, source) = self.unwrap();
        parsed.transform_with_source(source)
    }
}

impl<KS, KT, FX> TransformInto<BoxWise<KT, FX>> for BoxWise<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformWithSource<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> BoxWise<KT, FX> {
        BoxWise::from(self.unwrap().transform_into())
    }
}

impl<KS, KT, FX> TransformInto<Wise<KT, FX>> for Wise<KS, FX>
where
    KS: KastProvider,
    KT: KastProvider,
    KS::Expr<FX>: TransformWithSource<KT::Expr<FX>>,
    FX: Effect,
{
    fn transform_into(self) -> Wise<KT, FX> {
        Wise::from(self.unwrap().transform_into())
    }
}
