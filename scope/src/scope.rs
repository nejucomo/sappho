use sappho_identifier::RcId;
use sappho_list::List;
use sappho_value::Value;

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

    pub fn get(&self, key: &RcId) -> Option<&Value> {
        for locals in self.0.iter() {
            let sv = locals.get(key);
            if sv.is_some() {
                return sv;
            }
        }
        None
    }
}
