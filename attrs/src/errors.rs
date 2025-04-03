//! Errors for [Attrs](crate::Attrs) specific operations
use derive_more::From;
use derive_new::new;
use sappho_identifier::RcId;
use thiserror::Error;

#[derive(Clone, Debug, Error, new)]
#[error("attempt to redefine attr {attr}; existing {existing:?}; new {new:?}")]
pub struct Redefinition<T> {
    pub attr: RcId,
    pub existing: T,
    pub new: T,
}

#[derive(Clone, Debug, Error, From)]
#[error("missing {0:?}")]
#[from(RcId, &RcId)]
pub struct Missing(RcId);

#[derive(Clone, Debug, Error, From)]
#[error("unexpected attrs {0:?}")]
pub struct Unexpected(Vec<RcId>);

impl FromIterator<RcId> for Unexpected {
    fn from_iter<T: IntoIterator<Item = RcId>>(iter: T) -> Self {
        Unexpected(Vec::from_iter(iter))
    }
}
