use std::borrow::Borrow;
use std::rc::Rc;

use derive_more::From;
use sappho_unparse::Unparse;

use crate::{IdentRef, Identifier};

#[derive(Clone, Debug, From, Eq, Ord, PartialEq, PartialOrd)]
pub struct RcId(Rc<Identifier>);

impl From<Identifier> for RcId {
    fn from(id: Identifier) -> Self {
        RcId::from(Rc::from(id))
    }
}

impl Borrow<Identifier> for RcId {
    fn borrow(&self) -> &Identifier {
        self.0.borrow()
    }
}

impl Borrow<IdentRef> for RcId {
    fn borrow(&self) -> &IdentRef {
        let r: &Identifier = self.borrow();
        r.borrow()
    }
}

impl Unparse for RcId {
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        let idr: &IdentRef = self.borrow();
        idr.unparse_into(s)
    }
}
