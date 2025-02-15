use sappho_listform::ListForm;

use crate::IdentMap;

pub trait HeadTailUnrollable<X, T>: Sized {
    /// Translate a [ListForm] into a container which may contain [IdentMap]s with the Head/Tail Convention
    ///
    /// # The Head/Tail Convention
    ///
    /// This convention maps the empty list to the empty [IdentMap], and every element to `{"head": elem, "tail": tail}` converted into the target container type.
    fn unroll<V, ME, MT, WM>(self, map_elem: ME, map_tail: MT, wrap_map: WM) -> V
    where
        ME: Fn(X) -> V,
        MT: FnOnce(T) -> V,
        WM: Fn(IdentMap<V>) -> V;

    fn unroll_via_froms<V>(self) -> V
    where
        V: From<X> + From<T> + From<IdentMap<V>>,
    {
        self.unroll(V::from, V::from, V::from)
    }
}

impl<X, T> HeadTailUnrollable<X, T> for ListForm<X, T> {
    fn unroll<V, ME, MT, WM>(self, map_elem: ME, map_tail: MT, wrap_map: WM) -> V
    where
        ME: Fn(X) -> V,
        MT: FnOnce(T) -> V,
        WM: Fn(IdentMap<V>) -> V,
    {
        self.into_reverse_fold(
            |optail| {
                optail
                    .map(map_tail)
                    .unwrap_or_else(|| wrap_map(IdentMap::default()))
            },
            |tail, head| {
                wrap_map(IdentMap::from([
                    ("tail".to_string(), tail),
                    ("head".to_string(), map_elem(head)),
                ]))
            },
        )
    }
}
