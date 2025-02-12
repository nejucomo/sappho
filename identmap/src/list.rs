use derive_more::{From, Into};

/// This is a translation helper type for "unrolling" [ListForm] into an [IdentMap] with `"head"`, and `"tail"` keys
#[derive(Debug, From, Into)]
pub struct UnrolledList<T>(T);

impl<T, X, Z> From<ListForm<X, Z>> for IdentMap<T>
where
    T: Default + From<Z> + From<X> + From<Self>,
{
    fn from(lf: ListForm<X, Z>) -> Self {
        lf.into_reverse_fold(
            |optail| optail.map(T::from).unwrap_or_default(),
            |tail, head| {
                IdentMap::from_map([
                    ("tail".to_string(), T::from(tail)),
                    ("head".to_string(), T::from(head)),
                ])
            },
        )
    }
}
