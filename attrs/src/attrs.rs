use std::collections::BTreeMap;
use std::fmt::Debug;

use arrayvec::ArrayVec;
use either::Either::{self, Left, Right};
use sappho_identifier::{IdentRef, RcId};
use sappho_tfi::TryFromIterator;

use crate::errors::{Missing, Redefinition, Unexpected};

#[derive(Clone, Debug, PartialEq)]
pub struct Attrs<T>(BTreeMap<RcId, T>);

/// TODO: Change the `&IdentRef` looksup to `&RcId` after introducing an Identifier "interning" facility.
///
/// TODO: overhaul errors and apis
impl<T> Attrs<T>
where
    T: Debug,
{
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn define<K>(&mut self, id: K, val: T) -> Result<(), Redefinition<T>>
    where
        K: Into<RcId>,
    {
        let rcid = id.into();
        match self.0.insert(rcid.clone(), val) {
            None => Ok(()),
            Some(oldval) => {
                let newval = self.take(&rcid).unwrap();
                Err(Redefinition::new(rcid, oldval, newval))
            }
        }
    }

    pub fn define_many<I, K>(&mut self, pairs: I) -> Result<(), Redefinition<T>>
    where
        I: IntoIterator<Item = (K, T)>,
        K: Into<RcId>,
    {
        for (k, v) in pairs {
            self.define(k, v)?;
        }
        Ok(())
    }

    /// Define an entry then return `Self` on success
    ///
    /// # TODO
    ///
    /// Move the `S: Into<T>` clause to [Self::define]
    pub fn try_with<K, S>(mut self, id: K, val: S) -> Result<Self, Redefinition<T>>
    where
        K: Into<RcId>,
        S: Into<T>,
    {
        self.define(id, val.into())?;
        Ok(self)
    }

    /// Define an entry then return `Self`, panicking on error
    pub fn with<K, S>(self, id: K, val: S) -> Self
    where
        K: Into<RcId>,
        S: Into<T>,
    {
        self.try_with(id, val).unwrap()
    }

    /// Refer to the item stored at `key`
    pub fn get<K>(&self, key: K) -> Result<&T, Missing>
    where
        RcId: From<K>,
    {
        with_id(key, |id| self.0.get(id))
    }

    /// Refer to the item stored at `key`
    pub fn get_opt(&self, key: &RcId) -> Option<&T> {
        self.0.get(key)
    }

    /// Take the value(s) for the given `key`
    pub fn take<K>(&mut self, key: K) -> Result<T, Missing>
    where
        RcId: From<K>,
    {
        with_id(key, |id| self.0.remove(id))
    }

    /// Take the value(s) for the given `key` and ensure the remaining `self` is empty
    ///
    /// # TODO
    ///
    /// Review for removal.
    pub fn unpack<K, const N: usize>(mut self, keys: [K; N]) -> Either<[T; N], Self>
    where
        K: Into<RcId>,
    {
        let mut av = ArrayVec::default();
        for key in keys {
            let rcid = key.into();
            if let Ok(v) = self.take(&rcid) {
                av.push((rcid, v));
            } else {
                // Unwind mutations:
                self.define_many(av).unwrap();
                return Right(self);
            }
        }

        if self.is_empty() {
            Left(av.into_inner().unwrap().map(|(_, v)| v))
        } else {
            // Unwind mutations:
            self.define_many(av).unwrap();
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
    T: Clone + Debug,
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
        self.define(id, val)?;
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
        // BUG: does not check for duplicates; no error signal
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

fn with_id<K, F, T>(key: K, f: F) -> Result<T, Missing>
where
    RcId: From<K>,
    F: FnOnce(&IdentRef) -> Option<T>,
{
    let id = RcId::from(key);
    f(id.as_ref()).ok_or(Missing::from(id))
}
