use std::collections::BTreeMap;
use std::fmt::Debug;

use arrayvec::ArrayVec;
use either::Either::{self, Left, Right};
use sappho_identifier::RcId;
use sappho_tfi::TryFromIterator;

use crate::errors::{Missing, Redefinition, Unexpected};

#[derive(Clone, Debug, PartialEq)]
pub struct Attrs<T>(BTreeMap<RcId, T>);

/// TODO: Change the `&IdentRef` looksup to `&RcId` after introducing an Identifier "interning" facility.
impl<T> Attrs<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn define<K>(&mut self, id: K, val: T) -> Result<(), Redefinition<T>>
    where
        RcId: From<K>,
    {
        use std::collections::btree_map::Entry::{Occupied, Vacant};

        let rcid = RcId::from(id);
        match self.0.entry(rcid) {
            Vacant(vacation) => {
                vacation.insert(val);
                Ok(())
            }
            Occupied(entry) => {
                let (attr, existing) = entry.remove_entry();
                Err(Redefinition {
                    attr,
                    existing,
                    new: val,
                })
            }
        }
    }

    pub fn define_many<I, K>(&mut self, pairs: I) -> Result<(), Redefinition<T>>
    where
        I: IntoIterator<Item = (K, T)>,
        RcId: From<K>,
    {
        for (k, v) in pairs {
            self.define(k, v)?;
        }
        Ok(())
    }

    /// Get an output id
    pub fn get(&self, id: &RcId) -> Result<&T, Missing> {
        self.0.get(id).ok_or_else(|| Missing::from(id))
    }

    /// Take the value(s) for the given `key`
    pub fn take(&mut self, id: &RcId) -> Result<T, Missing> {
        self.0.remove(id).ok_or_else(|| Missing::from(id))
    }

    /// Take the value(s) for the given `key` and ensure the remaining `self` is empty
    pub fn unpack<K, const N: usize>(mut self, keys: [K; N]) -> Either<[T; N], Self>
    where
        T: std::fmt::Debug,
        RcId: From<K>,
    {
        let mut av = ArrayVec::default();
        for key in keys {
            let rcid = RcId::from(key);
            match self.take(&rcid) {
                Ok(v) => av.push((rcid, v)),
                Err(_) => {
                    // Unwind mutations:
                    self.define_many::<_, RcId>(av).unwrap();
                    return Right(self);
                }
            }
        }

        if self.is_empty() {
            Left(av.into_inner().unwrap().map(|(_, v)| v))
        } else {
            // Unwind mutations:
            self.define_many::<_, RcId>(av).unwrap();
            Right(self)
        }
    }

    pub fn expect_empty(self) -> Result<(), Unexpected> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self.0.into_keys().collect())
        }
    }

    pub fn as_refs(&self) -> Attrs<&T> {
        self.iter().collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RcId, &T)> {
        self.0.iter()
    }

    pub fn identifiers(&self) -> impl Iterator<Item = &RcId> {
        self.iter().map(|(idr, _)| idr)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.iter().map(|(_, v)| v)
    }

    pub fn map<F, U>(self, f: F) -> Attrs<U>
    where
        F: Fn(T) -> U,
    {
        self.into_iter().map(|(id, t)| (id, f(t))).collect()
    }
}

impl<T> Attrs<&T>
where
    T: Clone,
{
    pub fn cloned(self) -> Attrs<T>
    where
        T: Clone,
    {
        self.map(T::clone)
    }
}

impl<T> Default for Attrs<T> {
    fn default() -> Self {
        Attrs(BTreeMap::default())
    }
}

impl<S, T> TryFromIterator<(S, T)> for Attrs<T>
where
    S: Into<RcId>,
    T: Debug,
{
    type Error = Redefinition<T>;

    fn try_append(mut self, (id, val): (S, T)) -> Result<Self, Self::Error> {
        self.define(id.into(), val)?;
        Ok(self)
    }
}

/// # TODO
///
/// Remove in favor of `TryFromIterator`
impl<S, T> FromIterator<(S, T)> for Attrs<T>
where
    RcId: From<S>,
{
    fn from_iter<I: IntoIterator<Item = (S, T)>>(iter: I) -> Self {
        Attrs(iter.into_iter().map(|(s, v)| (RcId::from(s), v)).collect())
    }
}

impl<T> IntoIterator for Attrs<T> {
    type Item = (RcId, T);
    type IntoIter = <BTreeMap<RcId, T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
