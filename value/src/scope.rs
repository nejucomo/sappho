use sappho_attrs::errors::Missing;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_pattern::Pattern;

use crate::{Bind, BindError, Scoped, VResult, Value};

/// # TODO
///
/// Replace this with pre-runtime { RcId -> index } syntactic info and `Vec<Value>`
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scope(List<Locals>);

pub type Locals = Attrs<Value>;

impl Scope {
    pub fn wrap<T>(self, node: T) -> Scoped<T> {
        Scoped::new(self, node)
    }

    pub fn lookup(&self, key: &RcId) -> Result<&Value, Missing> {
        for locals in self.0.iter() {
            if let Ok(vref) = locals.get(key) {
                return Ok(vref);
            }
        }
        Err(Missing::from(key))
    }

    pub fn bind_call_scope(&self, bindings: &Pattern, arg: Value) -> VResult<Scope, BindError> {
        let locals = Locals::new_bindings(bindings, arg)?;
        Ok(Scope(self.0.prepend(locals)))
    }
}
