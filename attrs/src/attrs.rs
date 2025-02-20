use std::collections::BTreeMap;

use arrayvec::ArrayVec;
use sappho_identifier::{IdentRef, RcId};
use sappho_unparse::Unparse;

use crate::error::AttrsResult;
use crate::{AttrsError, Key};

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

    /// Get an output for any key, `K`, which includes `&IdentRef`
    ///
    /// Three common impls are `&IdentRef`, `&'static str`, and `(k1, k2)` which is a tuple of keys.
    ///
    /// For non-tuple keys, the output is just `&T`. For tuple keys the output is a tuple of the sub-key outputs.
    ///
    /// # Panics
    ///
    /// A `&'static str` key must be valid as an [IdentRef] and will cause a panic if not.
    ///
    /// # Performance
    ///
    /// This method is `self.as_refs().take(key)` which is nicely composable and terribly inefficient.
    pub fn get<K>(&self, key: K) -> AttrsResult<&T>
    where
        K: Key,
    {
        with_id(key, |id| self.0.get(id))
    }

    /// Take the value(s) for the given `key`
    ///
    /// See [Attrs::get] for the semantics of keys, their outputs, and panic conditions. However, the performance issue of [Attrs::get] is not present here.
    pub fn take<K>(&mut self, key: K) -> AttrsResult<T>
    where
        K: Key,
    {
        with_id(key, |id| self.0.remove(id))
    }

    /// Take the value(s) for the given `key` and ensure the remaining `self` is empty
    pub fn unpack<K, const N: usize>(mut self, keys: [K; N]) -> AttrsResult<[T; N]>
    where
        T: std::fmt::Debug,
        K: Key,
    {
        // First we ensure all keys are present before mutating:
        for k in keys.iter() {
            let idr = k.as_ident_ref();
            if !self.0.contains_key(idr) {
                return Err(AttrsError::Missing(RcId::from(idr)));
            }
        }

        let mut av = ArrayVec::default();
        for k in keys {
            av.push(self.take(k).unwrap());
        }

        Ok(av.into_inner().unwrap())
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

fn with_id<K, F, T>(key: K, f: F) -> AttrsResult<T>
where
    K: Key,
    F: FnOnce(&IdentRef) -> Option<T>,
{
    let id = key.as_ident_ref();
    f(id).ok_or_else(|| AttrsError::Missing(RcId::from(id)))
}
