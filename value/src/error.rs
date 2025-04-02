use sappho_attrs::AttrsError;
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

impl ValueErrorReason {
    pub fn with(self, value: Value) -> ValueError {
        ValueError {
            value,
            reason: self,
        }
    }
}
