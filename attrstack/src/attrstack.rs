use std::fmt::Debug;

use sappho_attrs::errors::Missing;
use sappho_attrs::Attrs;
use sappho_identifier::RcId;
use sappho_list::List;

#[derive(Clone, Debug, PartialEq)]
pub struct AttrStack<T>(List<Attrs<T>>)
where
    T: Debug;

impl<T> Default for AttrStack<T>
where
    T: Debug,
{
    fn default() -> Self {
        AttrStack(List::default())
    }
}

impl<T> AttrStack<T>
where
    T: Debug,
{
    pub fn push_attrs(self, locals: Attrs<T>) -> Self {
        AttrStack(self.0.prepend(locals))
    }

    pub fn get(&self, key: &RcId) -> Result<&T, Missing> {
        for attrs in self.0.iter() {
            let sv = attrs.get(key);
            if sv.is_ok() {
                return sv;
            }
        }
        Err(Missing::from(key))
    }
}
