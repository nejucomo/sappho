use sappho_list::List;
use sappho_primval::PrimVal;
use sappho_value::{Value, ValueError};

pub type BindResult<T> = Result<T, BindError>;

#[derive(Debug, thiserror::Error)]
pub enum BindError {
    #[error(transparent)]
    ValueError(#[from] ValueError),
    #[error("literal pattern mismatch: expected {0}, found {0}")]
    LitEqFailed(PrimVal, Value),
    #[error("unbound list tail: {0}")]
    UnboundTail(List<Value>),
}
