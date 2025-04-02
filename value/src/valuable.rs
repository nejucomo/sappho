use std::fmt::{Debug, Display};

use sappho_attrs::AttrsError;
use sappho_identifier::RcId;
use sappho_list::List;
use sappho_primval::PrimVal;

use crate::{VResult, Value, ValueErrorReason};

/// All of the user-space operations possible with a value
pub trait Valuable: Clone + Debug + Display + PartialEq + Into<Value> {
    fn clone_value(&self) -> Value {
        self.clone().into()
    }

    fn attr_lookup<'s>(&'s self, name: &RcId) -> VResult<&'s Value> {
        use AttrsError::Missing;
        use ValueErrorReason::Attrs;

        Err(Attrs(Missing(name.clone())).with(self.clone_value()))
    }

    fn as_list(&self) -> VResult<&List<Value>> {
        use ValueErrorReason::Type;

        Err(Type("a list").with(self.clone_value()))
    }
}

impl Valuable for PrimVal {}

impl Valuable for List<Value> {
    fn as_list(&self) -> VResult<&List<Value>> {
        Ok(self)
    }
}
