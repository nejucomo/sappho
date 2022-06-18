//! Provide deterministic mapping from identifiers to values

use std::collections::BTreeMap;

pub type Identifier = String;
pub type IdentRef = str;

#[derive(Debug, PartialEq)]
pub struct IdentMap<T>(BTreeMap<Identifier, T>);

#[derive(Debug)]
pub struct RedefinitionError(pub Identifier);

impl<T> IdentMap<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn define(&mut self, id: Identifier, val: T) -> Result<(), RedefinitionError> {
        // TODO: find a design that doesn't clone?
        match self.0.insert(id.clone(), val) {
            None => Ok(()),
            Some(_) => Err(RedefinitionError(id)),
        }
    }

    pub fn get(&self, id: &IdentRef) -> Option<&T> {
        self.0.get(id)
    }

    pub fn keys(&self) -> impl Iterator<Item = &Identifier> {
        self.0.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Identifier, &T)> {
        self.0.iter()
    }

    pub fn map_values<F, U>(self, f: F) -> IdentMap<U>
    where
        F: Fn(T) -> U,
    {
        let mut out = IdentMap::default();
        for (id, v) in self {
            out.define(id, f(v)).unwrap();
        }

        out
    }
}

impl<T> Default for IdentMap<T> {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl<T, F> From<F> for IdentMap<T>
where
    BTreeMap<Identifier, T>: From<F>,
{
    fn from(v: F) -> Self {
        IdentMap(BTreeMap::from(v))
    }
}

impl<T> IntoIterator for IdentMap<T> {
    type Item = (Identifier, T);
    type IntoIter = <BTreeMap<Identifier, T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
