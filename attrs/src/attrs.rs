use std::collections::BTreeMap;

use sappho_identifier::{IdentRef, RcId};

use crate::Redefinition;

#[derive(Clone, Debug)]
pub struct Attrs<T>(BTreeMap<RcId, T>);

impl<T> Attrs<T> {
    pub fn define<K>(&mut self, id: K, val: T) -> Result<(), Redefinition>
    where
        RcId: From<K>,
    {
        let rcid = RcId::from(id);
        match self.0.insert(rcid.clone(), val) {
            None => Ok(()),
            Some(_) => Err(Redefinition::from(rcid)),
        }
    }

    pub fn get(&self, id: &IdentRef) -> Option<&T> {
        self.0.get(id)
    }

    pub fn as_refs(&self) -> Attrs<&T> {
        self.iter().collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RcId, &T)> {
        self.0.iter()
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
