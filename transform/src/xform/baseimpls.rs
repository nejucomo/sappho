use sappho_attrs::Attrs;

use crate::xform::TransformInto;

impl<S, T> TransformInto<Attrs<T>> for Attrs<S>
where
    S: TransformInto<T>,
{
    fn transform(self) -> Attrs<T> {
        self.map(S::transform)
    }
}

impl<S, T> TransformInto<T> for Box<S>
where
    S: TransformInto<T>,
{
    fn transform(self) -> T {
        (*self).transform()
    }
}
