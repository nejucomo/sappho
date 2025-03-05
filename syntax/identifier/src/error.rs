use sappho_syntax_keyword::Keyword;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid identifier {candidate:?}: {reason}")]
pub struct InvalidIdentifier {
    candidate: String,
    reason: InvalidityReason,
}
aliri_braid::from_infallible!(InvalidIdentifier);

#[derive(Copy, Clone, Debug, Error)]
pub enum InvalidityReason {
    #[error("unacceptable character {0:?}")]
    BadChar(char),
    #[error("reserved keyword: {0:?}")]
    ReservedKeyword(Keyword),
}

impl InvalidityReason {
    pub fn for_input<S>(self, input: S) -> InvalidIdentifier
    where
        String: From<S>,
    {
        let candidate = String::from(input);
        let reason = self;
        InvalidIdentifier { candidate, reason }
    }
}
