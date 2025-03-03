use aliri_braid::braid;
use sappho_unparse::Unparse;

use crate::InvalidIdentifier;

#[braid(validator, ref_name = "IdentRef")]
pub struct Identifier;

impl aliri_braid::Validator for Identifier {
    type Error = InvalidIdentifier;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        for (ix, c) in raw.chars().enumerate() {
            let valid = if ix == 0 {
                // Only underscore or letters as the initial character:
                c == '_' || c.is_ascii_alphabetic()
            } else {
                // Underscore, letters, or digits for other characters:
                c == '_' || c.is_ascii_alphanumeric()
            };

            if !valid {
                return Err(InvalidIdentifier::from(raw));
            }
        }
        Ok(())
    }
}

impl Unparse for &IdentRef {
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        self.as_str().unparse_into(s)
    }
}
