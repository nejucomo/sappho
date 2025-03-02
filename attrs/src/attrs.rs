use std::collections::BTreeMap;

use arrayvec::ArrayVec;
use either::Either::{self, Left, Right};
use sappho_identifier::{IdentRef, RcId};
use sappho_unparse::Unparse;

use crate::error::AttrsResult;
use crate::AttrsError;

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

    pub fn define_many<I, K>(&mut self, pairs: I) -> AttrsResult<()>
    where
        I: IntoIterator<Item = (K, T)>,
        RcId: From<K>,
    {
        for (k, v) in pairs {
            self.define(k, v)?;
        }
        Ok(())
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
        RcId: From<K>,
    {
        with_id(key, |id| self.0.get(id))
    }

    /// Take the value(s) for the given `key`
    ///
    /// See [Attrs::get] for the semantics of keys, their outputs, and panic conditions. However, the performance issue of [Attrs::get] is not present here.
    pub fn take<K>(&mut self, key: K) -> AttrsResult<T>
    where
        RcId: From<K>,
    {
        with_id(key, |id| self.0.remove(id))
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
            match self.take::<&RcId>(&rcid) {
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
    RcId: From<K>,
    F: FnOnce(&IdentRef) -> Option<T>,
{
    let id = RcId::from(key);
    f(id.as_ref()).ok_or(AttrsError::Missing(id))
}
