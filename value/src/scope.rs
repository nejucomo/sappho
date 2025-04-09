use derive_more::Deref;
use sappho_attrs::Attrs;
use sappho_attrstack::AttrStack;

use crate::{Scoped, Value};

/// # TODO
///
/// Replace this with pre-runtime { RcId -> index } syntactic info and `Vec<Value>`
#[derive(Clone, Debug, Default, PartialEq, Deref)]
pub struct Scope(AttrStack<Value>);

pub type Locals = Attrs<Value>;

impl Scope {
    pub fn wrap<T>(self, node: T) -> Scoped<T> {
        Scoped::new(self, node)
    }
}
