use derive_new::new;

use crate::Scope;

#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct Scoped<T> {
    scope: Scope,
    data: T,
}
