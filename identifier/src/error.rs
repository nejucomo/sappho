use derive_more::From;
use thiserror::Error;

#[derive(Debug, From, Error)]
#[error("Invalid identifier: {0:?}")]
pub struct InvalidIdentifier(String);

aliri_braid::from_infallible!(InvalidIdentifier);

impl<'a> From<&'a str> for InvalidIdentifier {
    fn from(s: &'a str) -> Self {
        InvalidIdentifier::from(s.to_string())
    }
}
