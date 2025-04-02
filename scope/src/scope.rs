use sappho_list::List;

use crate::{Locals, Scoped};

#[derive(Clone, Debug, Default)]
pub struct Scope(List<Locals>);

impl Scope {
    pub fn wrap<T>(self, node: T) -> Scoped<T> {
        Scoped::new(self, node)
    }

    pub fn push_locals(self, locals: Locals) -> Self {
        Scope(self.0.prepend(locals))
    }
}
