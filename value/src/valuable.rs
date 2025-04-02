use std::fmt::{Debug, Display};

use sappho_attrs::AttrsError;
use sappho_identifier::RcId;
use sappho_primval::PrimVal;

use crate::Value;

/// All of the user-space operations possible with a value
pub trait Valuable: Clone + Debug + Display + PartialEq {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> Result<&'s Value, AttrsError>;
}

impl Valuable for PrimVal {
    fn attr_lookup<'s>(&'s self, name: &RcId) -> Result<&'s Value, AttrsError> {
        Err(AttrsError::Missing(name.clone()))
    }
}
