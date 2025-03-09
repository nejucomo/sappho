use aliri_braid::braid;
use sappho_keyword::Keyword;
use sappho_unparse::Unparse;

use crate::{InvalidIdentifier, InvalidityReason};

#[braid(validator, ref_name = "IdentRef")]
pub struct Identifier;

impl aliri_braid::Validator for Identifier {
    type Error = InvalidIdentifier;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        validate(raw).map_err(|reason| InvalidIdentifier::new(raw.to_string(), reason))
    }
}

impl Unparse for &IdentRef {
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.as_str().unparse_into(s)
    }
}

fn validate(raw: &str) -> Result<(), InvalidityReason> {
    use InvalidityReason::*;

    if raw.is_empty() {
        return Err(Empty);
    }

    for kw in Keyword::each() {
        if raw == kw.as_str() {
            return Err(ReservedKeyword(kw));
        }
    }

    for (ix, c) in raw.chars().enumerate() {
        let valid = if ix == 0 {
            // Only underscore or letters as the initial character:
            c == '_' || c.is_ascii_alphabetic()
        } else {
            // Underscore, letters, or digits for other characters:
            c == '_' || c.is_ascii_alphanumeric()
        };

        if !valid {
            if ix == 0 {
                return Err(ForbiddenInitialChar(c));
            } else {
                return Err(ForbiddenChar(c));
            }
        }
    }
    Ok(())
}
