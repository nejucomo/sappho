use derive_more::Deref;

#[derive(Debug, Deref)]
pub(crate) struct Ev<T>(pub(crate) T);
