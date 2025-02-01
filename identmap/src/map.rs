use crate::{IdentRef, Identifier, RedefinitionError, TryIntoIdentMap};
use sappho_legible::{Envelope, IntoNode, KeyValue, Node};
use sappho_listform::ListForm;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct IdentMap<T>(BTreeMap<Identifier, T>);

impl<T> IdentMap<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Identifier, &T)> {
        self.0.iter()
    }

    pub fn into_map_values<F, U>(self, f: F) -> IdentMap<U>
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

impl<T> IdentMap<T>
where
    T: TryIntoIdentMap<T>,
{
    pub fn as_list_form(&self) -> Option<ListForm<&T, &T>> {
        self.try_as_list_form().ok()
    }

    fn try_as_list_form(&self) -> Result<ListForm<&T, &T>, ()> {
        fn get<'a, T>(idmap: &'a IdentMap<T>, attr: &IdentRef) -> Result<&'a T, ()> {
            idmap.get(attr).ok_or(())
        }

        let mut ts = vec![];
        let mut idmap = self;
        loop {
            if idmap.is_empty() {
                return Ok(ListForm::new(ts, None));
            } else if idmap.len() != 2 {
                return Err(());
            }

            ts.push(get(idmap, "head")?);
            let tail = get(idmap, "tail")?;
            if let Some(tailmap) = tail.try_into_identmap() {
                idmap = tailmap;
            } else {
                return Ok(ListForm::new(ts, Some(tail)));
            }
        }
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

impl<T> FromIterator<(Identifier, T)> for IdentMap<T>
where
    BTreeMap<Identifier, T>: FromIterator<(Identifier, T)>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Identifier, T)>,
    {
        IdentMap(BTreeMap::from_iter(iter))
    }
}

impl<T> IntoIterator for IdentMap<T> {
    type Item = (Identifier, T);
    type IntoIter = <BTreeMap<Identifier, T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'s, T> IntoNode for &'s IdentMap<T>
where
    &'s T: IntoNode,
{
    fn into_node(self) -> Node {
        Envelope::separated_bracketed_sequence("{", ",", "}", self.iter().map(KeyValue::from))
            .into_node()
    }
}
