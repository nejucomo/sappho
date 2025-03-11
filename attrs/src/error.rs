use std::convert::Infallible;

use sappho_identifier::{InvalidIdentifier, RcId};
use thiserror::Error;

pub type AttrsResult<T> = Result<T, AttrsError>;

#[derive(Clone, Debug, Error)]
pub enum AttrsError {
    #[error(transparent)]
    InvalidIdentifier(#[from] InvalidIdentifier),
    #[error("attempt to redefine attr {0:?}")]
    Redefinition(RcId),
    #[error("missing expected attr {0:?}")]
    Missing(RcId),
    #[error("unexpected attrs {0:?}")]
    Unexpected(Vec<RcId>),
}

impl From<Infallible> for AttrsError {
    fn from(_: Infallible) -> Self {
        panic!("someone instantiated infallible!!!!");
    }
}
