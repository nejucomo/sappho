//! Provide deterministic mapping from identifiers to values

mod tryinto;

pub use self::tryinto::TryIntoIdentMap;

use std::collections::BTreeMap;
use std::fmt;

pub type Identifier = String;
pub type IdentRef = str;

#[derive(Clone, Debug, PartialEq)]
pub struct IdentMap<T>(BTreeMap<Identifier, T>);

#[derive(Debug)]
pub struct RedefinitionError(pub Identifier);

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

    pub fn map_value_refs<F, U>(&self, f: F) -> IdentMap<U>
    where
        F: Fn(&T) -> U,
    {
        let mut out = IdentMap::default();
        for (id, v) in self.iter() {
            out.define(id.clone(), f(v)).unwrap();
        }

        out
    }
}

impl<T> IdentMap<T>
where
    T: TryIntoIdentMap<T>,
{
    pub fn as_list_form(&self) -> Option<(Vec<&T>, Option<&T>)> {
        self.try_as_list_form().ok()
    }

    fn try_as_list_form(&self) -> Result<(Vec<&T>, Option<&T>), ()> {
        fn get<'a, T>(idmap: &'a IdentMap<T>, attr: &IdentRef) -> Result<&'a T, ()> {
            idmap.get(attr).ok_or(())
        }

        let mut ts = vec![];
        let mut idmap = self;
        loop {
            if idmap.is_empty() {
                return Ok((ts, None));
            } else if idmap.len() != 2 {
                return Err(());
            }

            ts.push(get(idmap, "head")?);
            let tail = get(idmap, "tail")?;
            if let Some(tailmap) = tail.try_into_identmap() {
                idmap = tailmap;
            } else {
                return Ok((ts, Some(tail)));
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

impl<T> fmt::Display for IdentMap<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "{{}}")
        } else {
            use sappho_fmtutil::fmt_comma_sep;

            write!(f, "{{ ")?;
            fmt_comma_sep(self.0.iter().map(|(n, a)| format!("{}: {}", n, a)), f)?;
            write!(f, " }}")
        }
    }
}
