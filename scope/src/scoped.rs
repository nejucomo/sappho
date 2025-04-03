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

impl<T> Scoped<T> {
    pub fn map<F, U>(self, f: F) -> Scoped<U>
    where
        F: FnOnce(T) -> U,
    {
        Scoped {
            scope: self.scope,
            node: f(self.node),
        }
    }
}
