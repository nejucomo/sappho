use std::collections::BTreeMap;

use sappho_identifier::{IdentRef, RcId};
use sappho_unparse::Unparse;

use crate::error::AttrsResult;
use crate::{AttrsError, AttrsTailAdapter, HeadAndTailIter};

#[derive(Clone, Debug, PartialEq)]
pub struct Attrs<T>(BTreeMap<RcId, T>);

/// TODO: Change the `&IdentRef` looksup to `&RcId` after introducing an Identifier "interning" facility.
impl<T> Attrs<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn define<K>(&mut self, id: K, val: T) -> AttrsResult<()>
    where
        RcId: From<K>,
    {
        let rcid = RcId::from(id);
        match self.0.insert(rcid.clone(), val) {
            None => Ok(()),
            Some(_) => Err(AttrsError::Redefinition(rcid)),
        }
    }

    pub fn get(&self, id: &IdentRef) -> AttrsResult<&T> {
        self.0
            .get(id)
            .ok_or_else(|| AttrsError::Missing(RcId::from(id)))
    }

    pub fn remove(&mut self, id: &IdentRef) -> AttrsResult<T> {
        self.0
            .remove(id)
            .ok_or_else(|| AttrsError::Missing(RcId::from(id)))
    }

    pub fn expect_empty(self) -> AttrsResult<()> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(AttrsError::Unexpected(self.0.into_keys().collect()))
        }
    }

    pub fn as_refs(&self) -> Attrs<&T> {
        self.iter().collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RcId, &T)> {
        self.0.iter()
    }

    pub fn map<F, U>(self, f: F) -> Attrs<U>
    where
        F: Fn(T) -> U,
    {
        self.into_iter().map(|(id, t)| (id, f(t))).collect()
    }

    pub fn into_head_and_tail_iter(self) -> HeadAndTailIter<T>
    where
        T: AttrsTailAdapter,
    {
        HeadAndTailIter::from(self)
    }
}

impl<T> Default for Attrs<T> {
    fn default() -> Self {
        Attrs(BTreeMap::default())
    }
}

impl<T> FromIterator<(RcId, T)> for Attrs<T> {
    fn from_iter<I: IntoIterator<Item = (RcId, T)>>(iter: I) -> Self {
        Attrs(iter.into_iter().collect())
    }
}

impl<'a, T> FromIterator<(&'a RcId, &'a T)> for Attrs<&'a T> {
    fn from_iter<I: IntoIterator<Item = (&'a RcId, &'a T)>>(iter: I) -> Self {
        iter.into_iter()
            .map(|(rcid, tref)| (rcid.clone(), tref))
            .collect()
    }
}

impl<T> IntoIterator for Attrs<T> {
    type Item = (RcId, T);
    type IntoIter = <BTreeMap<RcId, T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Unparse for Attrs<T>
where
    T: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        use sappho_unparse::{Brackets::Squiggle, Break::OptSpace};

        if self.0.is_empty() {
            s.write("{}");
        } else {
            s.bracketed(Squiggle, |subs| {
                for (k, v) in self.iter() {
                    subs.write(&OptSpace);
                    subs.write(k);
                    subs.write(": ");
                    subs.write(v);
                    subs.write(",");
                }
            });
        }
    }
}
