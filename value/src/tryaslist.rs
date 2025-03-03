use std::borrow::Borrow;

use sappho_listform::ListForm;
use sappho_object::Unbundled;

use crate::{AttrVals, Object, ValRef, Value};

pub(crate) trait TryAsList {
    fn try_as_list(&self) -> Option<ListForm<ValRef, ValRef>>;
}

impl TryAsList for AttrVals {
    fn try_as_list(&self) -> Option<ListForm<ValRef, ValRef>> {
        if self.is_empty() {
            Some(ListForm::default())
        } else {
            let [head, tail] = self.as_refs().unpack(["head", "tail"]).left()?;
            let lf = tail.try_as_list()?;
            Some(lf.prepend(head.clone()))
        }
    }
}

impl TryAsList for ValRef {
    fn try_as_list(&self) -> Option<ListForm<ValRef, ValRef>> {
        let v: &Value = self.borrow();
        v.try_as_list()
    }
}

impl TryAsList for Value {
    fn try_as_list(&self) -> Option<ListForm<ValRef, ValRef>> {
        match self {
            Value::Num(_) => None,
            Value::Object(object) => object.try_as_list(),
        }
    }
}

impl<T> TryAsList for Box<T>
where
    T: TryAsList,
{
    fn try_as_list(&self) -> Option<ListForm<ValRef, ValRef>> {
        self.as_ref().try_as_list()
    }
}

impl TryAsList for Object {
    fn try_as_list(&self) -> Option<ListForm<ValRef, ValRef>> {
        match self.as_refs().unbundle() {
            Unbundled::Attrs(attrs) => {
                let attrs: AttrVals = attrs.cloned();
                attrs.try_as_list()
            }
            _ => None,
        }
    }
}
