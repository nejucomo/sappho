use derive_new::new;

use crate::Scope;

#[derive(Debug, new)]
pub struct Scoped<T> {
    pub scope: Scope,
    pub node: T,
}

impl<T> From<T> for Scoped<T> {
    fn from(node: T) -> Self {
        Self::new(Scope::default(), node)
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

impl<T, E> Scoped<Result<T, E>> {
    pub fn transpose(self) -> Result<Scoped<T>, E> {
        self.node.map(|t| self.scope.wrap(t))
    }
}
