use derive_more::From;
use sappho_identifier::RcId;
use thiserror::Error;

#[derive(Clone, Debug, Error, From)]
#[error("redefinition of {0:?}")]
pub struct Redefinition(RcId);
