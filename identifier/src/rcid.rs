use std::borrow::Borrow;
use std::rc::Rc;

use derive_more::From;

use crate::{IdentRef, Identifier};

#[derive(Clone, Debug, From, Eq, Ord, PartialEq, PartialOrd)]
pub struct RcId(Rc<Identifier>);

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
