use derive_new::new;
use sappho_value::Scope;

#[derive(Debug, new)]
pub(crate) struct Scoped<T> {
    pub(crate) scope: Scope,
    pub(crate) node: T,
}

impl<T> From<T> for Scoped<T> {
    fn from(node: T) -> Self {
        Self::new(Scope::default(), node)
    }
}

impl<T> Scoped<T> {
    pub(crate) fn map<F, U>(self, f: F) -> Scoped<U>
    where
        F: FnOnce(T) -> U,
    {
        Scoped {
            scope: self.scope,
            node: f(self.node),
        }
    }
}
