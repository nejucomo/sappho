use sappho_identifier::RcId;
use thiserror::Error;

pub type AttrsResult<T> = Result<T, AttrsError>;

#[derive(Clone, Debug, Error)]
pub enum AttrsError {
    #[error("attempt to redefine attr {0:?}")]
    Redefinition(RcId),
    #[error("missing expected attr {0:?}")]
    Missing(RcId),
    #[error("unexpected attrs {0:?}")]
    Unexpected(Vec<RcId>),
}
