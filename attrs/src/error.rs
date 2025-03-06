use sappho_syntax_idstore::ArcId;
use thiserror::Error;

pub type AttrsResult<T> = Result<T, AttrsError>;

#[derive(Clone, Debug, Error)]
pub enum AttrsError {
    #[error("attempt to redefine attr {0:?}")]
    Redefinition(ArcId),
    #[error("missing expected attr {0:?}")]
    Missing(ArcId),
    #[error("unexpected attrs {0:?}")]
    Unexpected(Vec<ArcId>),
}
