use std::borrow::Borrow;
use std::fmt;
use std::rc::Rc;

use derive_more::From;
use sappho_unparse::Unparse;

use crate::{IdentRef, Identifier};

#[derive(Clone, Debug, From, Eq, Ord, PartialEq, PartialOrd)]
pub struct RcId(Rc<Identifier>);

impl RcId {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl TryFrom<String> for RcId {
    type Error = <Identifier as TryFrom<String>>::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Identifier::try_from(s).map(RcId::from)
    }
}

impl From<Identifier> for RcId {
    fn from(id: Identifier) -> Self {
        RcId::from(Rc::from(id))
    }
}

impl<'a> From<&'a RcId> for RcId {
    fn from(rcid: &'a RcId) -> Self {
        (*rcid).clone()
    }
}

impl<'a> From<&'a IdentRef> for RcId {
    fn from(idr: &'a IdentRef) -> Self {
        RcId::from(Identifier::from(idr))
    }
}

impl From<&'static str> for RcId {
    fn from(s: &'static str) -> Self {
        RcId::from(Identifier::from_static(s))
    }
}

impl Borrow<IdentRef> for RcId {
    fn borrow(&self) -> &IdentRef {
        let r: &Identifier = self.0.borrow();
        r.borrow()
    }
}

impl AsRef<IdentRef> for RcId {
    fn as_ref(&self) -> &IdentRef {
        self.borrow()
    }
}

impl AsRef<str> for RcId {
    fn as_ref(&self) -> &str {
        let id: &IdentRef = self.as_ref();
        id.as_str()
    }
}

impl Unparse for RcId {
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        let idr: &IdentRef = self.borrow();
        idr.unparse_into(s)
    }
}

impl fmt::Display for RcId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idr: &IdentRef = self.borrow();
        idr.fmt(f)
    }
}
