use std::fmt::{Debug, Display};

use sappho_attrs::errors::Missing;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_primval::PrimVal;
use thiserror::Error;

use crate::{VResult, Value, ValueError};

#[derive(Debug, Error)]
#[error("could not adapt value as {0}")]
pub struct AsError(&'static str);

/// All of the user-space operations possible with a value
pub trait Valuable: Clone + Debug + Display + PartialEq + Into<Value> {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> VResult<&'s Value, Missing> {
        Err(self.wrap_error(Missing::from(name)))
    }

    fn as_list(&self) -> VResult<&List<Value>, AsError> {
        Err(self.wrap_error(AsError("list")))
    }

    fn clone_into_value(&self) -> Value {
        self.clone().into()
    }

    fn wrap_error<E>(&self, inner: E) -> ValueError<E> {
        ValueError::new(self.clone_into_value(), inner)
    }
}

impl Valuable for PrimVal {}

impl Valuable for List<Value> {
    fn as_list(&self) -> VResult<&List<Value>, AsError> {
        Ok(self)
    }
}
