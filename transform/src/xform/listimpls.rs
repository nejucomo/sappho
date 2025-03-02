use std::fmt::Debug;

use either::Either::{self, Left, Right};
use sappho_attrs::Attrs;
use sappho_listform::ListForm;

use crate::xform::{TransformInto, TryTransformInto};

impl<V, X, T> TransformInto<V> for ListForm<X, T>
where
    V: From<Attrs<V>>,
    X: TransformInto<V>,
    T: TransformInto<V>,
{
    fn transform(self) -> V {
        self.into_iter()
            .rev()
            .fold(V::from(Attrs::default()), |red, asteither| {
                asteither
                    .map_right(T::transform)
                    .map_left(|x| {
                        V::from(Attrs::from_iter([("head", x.transform()), ("tail", red)]))
                    })
                    .into_inner()
            })
    }
}

impl<V, X, T> TryTransformInto<ListForm<X, T>> for Attrs<V>
where
    V: TransformInto<X> + TryTransformInto<Either<T, Attrs<V>>> + From<Attrs<V>> + Debug,
{
    fn try_transform(self) -> Either<ListForm<X, T>, Self> {
        if self.is_empty() {
            Left(ListForm::default())
        } else {
            self.unpack(["head", "tail"])
                .left_and_then(|[head, vtail]| {
                    let ei = vtail
                        .try_transform()
                        .left_and_then(|ei: Either<T, Attrs<V>>| {
                            ei.either(
                                |t| Left(ListForm::new(None, Some(t))),
                                |attrs| attrs.try_transform(),
                            )
                            .map_right(V::from)
                        });

                    match ei {
                        Left(lf) => Left(lf.prepend(head.transform())),
                        Right(vtail) => Right(Attrs::from_iter([("head", head), ("tail", vtail)])),
                    }
                })
        }
    }
}
