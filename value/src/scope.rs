use derive_more::Deref;
use sappho_attrs::errors::Missing;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_list::List;

use crate::Value;

/// # TODO
///
/// Replace this with pre-runtime { RcId -> index } syntactic info and `Vec<Value>`
#[derive(Clone, Debug, Default, PartialEq, Deref)]
pub struct Scope(List<Locals>);

pub type Locals = Attrs<Value>;

impl Scope {
    pub fn lookup(&self, binding: &RcId) -> Result<&Value, Missing> {
        for locals in self.0.iter() {
            if let Ok(v) = locals.get(binding) {
                return Ok(v);
            }
        }
        Err(Missing::from(binding))
    }

    pub fn push_locals(&self, locals: Locals) -> Scope {
        Scope(self.0.clone().prepend(locals))
    }

    pub fn pop_locals(&self) -> Option<(Scope, &Locals)> {
        self.0
            .next()
            .map(|(lref, localsref)| (Scope(lref.clone()), localsref))
    }
}
