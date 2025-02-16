use aliri_braid::braid;

use crate::InvalidIdentifier;

#[braid(validator, ref_name = "IdentRef")]
pub struct Identifier;

impl aliri_braid::Validator for Identifier {
    type Error = InvalidIdentifier;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        for c in raw.chars() {
            if c != '_' && !c.is_ascii_alphanumeric() {
                return Err(InvalidIdentifier::from(raw));
            }
        }
        Ok(())
    }
}
