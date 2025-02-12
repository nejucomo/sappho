use derive_more::From;
use sappho_listform::ListForm;

use crate::IdentMap;

/// This is a translation helper type for "unrolling" [ListForm] into an [IdentMap] with `"head"`, and `"tail"` keys
#[derive(Debug, From)]
pub struct ListUnroll<T>(T);

impl<T> ListUnroll<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, X, Z> From<ListForm<X, Z>> for ListUnroll<T>
where
    T: From<Z> + From<X> + From<IdentMap<T>>,
{
    fn from(lf: ListForm<X, Z>) -> Self {
        ListUnroll(lf.into_reverse_fold(
            |optail| {
                optail
                    .map(T::from)
                    .unwrap_or_else(|| T::from(IdentMap::default()))
            },
            |tail, head| {
                T::from(IdentMap::from([
                    ("tail".to_string(), tail),
                    ("head".to_string(), T::from(head)),
                ]))
            },
        ))
    }
}
