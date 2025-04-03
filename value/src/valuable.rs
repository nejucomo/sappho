use std::fmt::{Debug, Display};

use sappho_attrs::Missing;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_primval::PrimVal;

use crate::{VResult, Value, ValueError};

/// All of the user-space operations possible with a value
pub trait Valuable: Clone + Debug + Display + PartialEq + Into<Value> {
    fn clone_into_value(&self) -> Value {
        self.clone().into()
    }

    fn wrap_error<E>(&self, inner: E) -> ValueError<E> {
        ValueError::new(self.clone_into_value(), inner)
    }

    fn attr_lookup<'s>(&'s self, name: &RcId) -> Result<&'s Value, ValueError<Missing>> {
        Err(self.wrap_error(Missing::from(name)))
    }

    fn as_list(&self) -> VResult<&List<Value>> {
        use ValueErrorReason::Type;

        Err(Type("a list").with(self.clone_into_value()))
    }
}

impl Valuable for PrimVal {}

impl Valuable for List<Value> {
    fn as_list(&self) -> VResult<&List<Value>> {
        Ok(self)
    }
}
