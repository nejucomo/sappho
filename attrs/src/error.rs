use std::convert::Infallible;

use derive_new::new;
use sappho_identifier::{InvalidIdentifier, RcId};
use thiserror::Error;

pub type AttrsResult<O, T> = Result<O, AttrsError<T>>;

#[derive(Clone, Debug, Error)]
pub enum AttrsError<T> {
    #[error(transparent)]
    InvalidIdentifier(#[from] InvalidIdentifier),
    #[error(transparent)]
    Redefinition(#[from] Redefinition<T>),
    #[error("missing expected attr {0:?}")]
    Missing(RcId),
    #[error("unexpected attrs {0:?}")]
    Unexpected(Vec<RcId>),
}

impl<T> From<Infallible> for AttrsError<T> {
    fn from(_: Infallible) -> Self {
        panic!("someone instantiated infallible!!!!");
    }
}

#[derive(Clone, Debug, Error, new)]
#[error("attempt to redefine attr {attr}; existing {existing:?}; new {new:?}")]
pub struct Redefinition<T> {
    pub attr: RcId,
    pub existing: T,
    pub new: T,
}
