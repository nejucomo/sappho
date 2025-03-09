use derive_new::new;
use sappho_keyword::Keyword;
use thiserror::Error;

#[derive(Debug, Error, new)]
#[error("Invalid identifier {candidate:?}: {reason}")]
pub struct InvalidIdentifier {
    pub candidate: String,
    pub reason: InvalidityReason,
}

aliri_braid::from_infallible!(InvalidIdentifier);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum InvalidityReason {
    #[error("identifiers must have at least one character")]
    Empty,
    #[error("identifiers must not begin char {0:?}")]
    ForbiddenInitialChar(char),
    #[error("identifiers must not include forbidden char {0:?}")]
    ForbiddenChar(char),
    #[error("identifiers must not be the reserved keyword {0}")]
    ReservedKeyword(Keyword),
}
