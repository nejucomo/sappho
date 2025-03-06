use std::borrow::Borrow;
use std::fmt;
use std::sync::Arc;

use derive_more::From;
use sappho_syntax_parsable::Parsable;
use sappho_syntax_unparse::Unparse;

use crate::{resolve, IdentRef, Identifier, InvalidIdentifier};

#[derive(Clone, Debug, From, Eq, Ord, PartialEq, PartialOrd)]
pub struct ArcId(Arc<Identifier>);

impl ArcId {
    pub(crate) fn new(s: String) -> Result<Self, InvalidIdentifier> {
        Identifier::try_from(s).map(Arc::new).map(ArcId)
    }

    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl<'a> From<&'a ArcId> for ArcId {
    fn from(rcid: &'a ArcId) -> Self {
        (*rcid).clone()
    }
}

impl Borrow<IdentRef> for ArcId {
    fn borrow(&self) -> &IdentRef {
        let r: &Identifier = self.0.borrow();
        r.borrow()
    }
}

impl AsRef<IdentRef> for ArcId {
    fn as_ref(&self) -> &IdentRef {
        self.borrow()
    }
}

impl AsRef<str> for ArcId {
    fn as_ref(&self) -> &str {
        let id: &IdentRef = self.as_ref();
        id.as_str()
    }
}

impl Parsable for ArcId {
    fn parser() -> impl sappho_syntax_parsable::Parser<Self> {
        use chumsky::{text, Parser};
        use sappho_syntax_parsable::error::ChumskyError;

        text::ident().try_map(|text, span| resolve(text).map_err(|e| ChumskyError::custom(span, e)))
    }
}

impl Unparse for ArcId {
    fn unparse_into(&self, s: &mut sappho_syntax_unparse::Stream) {
        let idr: &IdentRef = self.borrow();
        idr.unparse_into(s)
    }
}

impl fmt::Display for ArcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idr: &IdentRef = self.borrow();
        idr.fmt(f)
    }
}
