use derive_new::new;

use crate::Scope;

#[derive(Clone, Debug, new)]
pub struct Scoped<T> {
    pub scope: Scope,
    pub node: T,
}

impl<T> From<T> for Scoped<T> {
    fn from(node: T) -> Self {
        Scoped::new(Scope::default(), node)
    }
}
