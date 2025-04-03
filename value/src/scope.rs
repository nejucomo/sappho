use derive_more::Deref;
use sappho_attrs::Attrs;
use sappho_attrstack::AttrStack;

use crate::Value;

#[derive(Clone, Debug, Default, PartialEq, Deref)]
pub struct Scope(AttrStack<Value>);

pub type Locals = Attrs<Value>;
