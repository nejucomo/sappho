use aliri_braid::braid;
use sappho_syntax_keyword::Keyword;
use sappho_syntax_parsable::error::ChumskyError;
use sappho_syntax_parsable::Parsable;
use sappho_syntax_unparse::Unparse;

use crate::error::InvalidityReason;
use crate::InvalidIdentifier;

#[braid(validator, ref_name = "IdentRef")]
pub struct Identifier;

impl Identifier {
    pub fn as_idref(&self) -> &IdentRef {
        self.as_ref()
    }
}

impl aliri_braid::Validator for Identifier {
    type Error = InvalidIdentifier;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        validate_inner(raw).map_err(|inv| inv.for_input(raw))
    }
}

impl Parsable for Identifier {
    fn parser() -> impl sappho_syntax_parsable::Parser<Self> {
        use chumsky::{text, Parser};

        text::ident().try_map(|text, span| {
            Identifier::try_from(text).map_err(|e| ChumskyError::custom(span, e))
        })
    }
}

impl Unparse for Identifier {
    fn unparse_into(&self, s: &mut sappho_syntax_unparse::Stream) {
        self.as_idref().unparse_into(s)
    }
}

impl Unparse for &IdentRef {
    fn unparse_into(&self, s: &mut sappho_syntax_unparse::Stream) {
        self.as_str().unparse_into(s)
    }
}

fn validate_inner(raw: &str) -> Result<(), InvalidityReason> {
    use InvalidityReason::*;

    for (ix, c) in raw.chars().enumerate() {
        let valid = if ix == 0 {
            // Only underscore or letters as the initial character:
            c == '_' || c.is_ascii_alphabetic()
        } else {
            // Underscore, letters, or digits for other characters:
            c == '_' || c.is_ascii_alphanumeric()
        };

        if !valid {
            return Err(BadChar(c));
        }
    }

    if let Ok(kw) = Keyword::try_from(raw) {
        Err(ReservedKeyword(kw))
    } else {
        Ok(())
    }
}
