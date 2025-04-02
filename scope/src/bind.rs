use sappho_attrs::AttrsError;
use sappho_primval::PrimVal;
use sappho_value::Value;

pub type BindResult<T> = Result<T, BindError>;

#[derive(Debug, thiserror::Error)]
pub enum BindError {
    #[error(transparent)]
    AttrsError(#[from] AttrsError),
    #[error("literal pattern mismatch: expected {0}, found {0}")]
    LitEqFailed(PrimVal, Value),
}
