use sappho_effect::Effect;
use sappho_syntax as syntax;
use sappho_with_source::WithSource;

use crate::transform::TransformWithSource;
use crate::{BoxWise, Wise};

pub(crate) trait TransformInto<T> {
    fn transform_into(self) -> T;
}

impl<S, T> TransformInto<T> for WithSource<S>
where
    S: TransformWithSource<T>,
{
    fn transform_into(self) -> T {
        let (parsed, source) = self.unwrap();
        parsed.transform_with_source(&source)
    }
}

impl<S, T> TransformInto<Vec<T>> for Vec<S>
where
    S: TransformInto<T>,
{
    fn transform_into(self) -> Vec<T> {
        self.into_iter().map(S::transform_into).collect()
    }
}

impl<FX> TransformInto<BoxWise<FX>> for syntax::BoxWise<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> BoxWise<FX> {
        BoxWise::from(self.unwrap().transform_into())
    }
}

impl<FX> TransformInto<Wise<FX>> for syntax::Wise<FX>
where
    FX: Effect,
{
    fn transform_into(self) -> Wise<FX> {
        Wise::from(self.unwrap().transform_into())
    }
}
