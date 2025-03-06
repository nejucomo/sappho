use std::collections::BTreeMap;

use arrayvec::ArrayVec;
use either::Either::{self, Left, Right};
use sappho_syntax_idstore::ArcId;
use sappho_syntax_unparse::Unparse;

use crate::error::AttrsResult;
use crate::AttrsError;

#[derive(Clone, Debug, PartialEq)]
pub struct Attrs<T>(BTreeMap<ArcId, T>);

impl<T> Attrs<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn define(&mut self, id: ArcId, val: T) -> AttrsResult<()> {
        match self.0.insert(id.clone(), val) {
            None => Ok(()),
            Some(_) => Err(AttrsError::Redefinition(id)),
        }
    }

    pub fn define_many<'a, I>(&mut self, pairs: I) -> AttrsResult<()>
    where
        I: IntoIterator<Item = (&'a ArcId, T)>,
    {
        for (k, v) in pairs {
            self.define(k.clone(), v)?;
        }
        Ok(())
    }

    /// Get the value with the given key
    pub fn get(&self, idref: &ArcId) -> AttrsResult<&T> {
        with_id(idref, |id| self.0.get(id))
    }

    /// Take the value(s) for the given `key`
    pub fn take(&mut self, idref: &ArcId) -> AttrsResult<T> {
        with_id(idref, |id| self.0.remove(id))
    }

    /// Take the value(s) for the given `idrefs` and ensure the remaining `self` is empty
    pub fn unpack<const N: usize>(mut self, idrefs: [&ArcId; N]) -> Either<[T; N], Self>
    where
        T: std::fmt::Debug,
    {
        let mut av = ArrayVec::default();
        for idref in idrefs {
            match self.take(idref) {
                Ok(v) => av.push((idref, v)),
                Err(_) => {
                    // Unwind mutations:
                    self.define_many(av).unwrap();
                    return Right(self);
                }
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

    pub fn iter(&self) -> impl Iterator<Item = (&ArcId, &T)> {
        self.0.iter()
    }

    pub fn identifiers(&self) -> impl Iterator<Item = &ArcId> {
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

impl<K, T> FromIterator<(K, T)> for Attrs<T>
where
    ArcId: From<K>,
{
    fn from_iter<I: IntoIterator<Item = (K, T)>>(iter: I) -> Self {
        Attrs(iter.into_iter().map(|(k, v)| (ArcId::from(k), v)).collect())
    }
}

impl<T> IntoIterator for Attrs<T> {
    type Item = (ArcId, T);
    type IntoIter = <BTreeMap<ArcId, T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Unparse for Attrs<T>
where
    T: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_syntax_unparse::Stream) {
        use sappho_syntax_unparse::{Brackets::Squiggle, Break::OptSpace};

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

fn with_id<F, T>(idref: &ArcId, f: F) -> AttrsResult<T>
where
    F: FnOnce(&ArcId) -> Option<T>,
{
    f(idref).ok_or_else(|| AttrsError::Missing(idref.clone()))
}
