use derive_more::Deref;
use sappho_attrs::errors::Missing;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_list::List;

use crate::{Scoped, Value};

/// # TODO
///
/// Replace this with pre-runtime { RcId -> index } syntactic info and `Vec<Value>`
#[derive(Clone, Debug, Default, PartialEq, Deref)]
pub struct Scope(List<Locals>);

pub type Locals = Attrs<Value>;

impl Scope {
    pub fn lookup(&self, id: &RcId) -> Result<&Value, Missing> {
        for locals in self.0.iter() {
            if let Ok(vref) = locals.get(id) {
                return Ok(vref);
            }
        }

        Err(Missing::from(id))
    }

    pub fn wrap<T>(self, other: T) -> Scoped<T> {
        Scoped::new(self, other)
    }

    pub fn clone_wrap<T>(&self, other: T) -> Scoped<T> {
        self.clone().wrap(other)
    }
}
