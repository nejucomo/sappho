use sappho_attrs::{AttrsError, Redefinition};
use thiserror::Error;

use crate::Value;

pub type VResult<T> = Result<T, ValueError>;

#[derive(Debug, Error)]
#[error("value error - {reason}; for value: {value}")]
pub struct ValueError {
    value: Value,
    reason: ValueErrorReason,
}

#[derive(Debug, Error)]
pub enum ValueErrorReason {
    #[error("expected {0}")]
    Type(&'static str),
    #[error(transparent)]
    Attrs(#[from] AttrsError<Value>),
}

impl TryFrom<AttrsError<Value>> for ValueError {
    type Error = AttrsError<Value>;

    fn try_from(ae: AttrsError<Value>) -> Result<Self, Self::Error> {
        match ae {
            AttrsError::Redefinition(x) => Ok(x.into()),
            other => Err(other),
        }
    }
}

impl From<Redefinition<Value>> for ValueError {
    fn from(red: Redefinition<Value>) -> Self {
        ValueError {
            value: red.new.clone(),
            reason: red.into(),
        }
    }
}

impl ValueErrorReason {
    pub fn with(self, value: Value) -> ValueError {
        ValueError {
            value,
            reason: self,
        }
    }
}

impl From<Redefinition<Value>> for ValueErrorReason {
    fn from(red: Redefinition<Value>) -> Self {
        ValueErrorReason::Attrs(red.into())
    }
}
