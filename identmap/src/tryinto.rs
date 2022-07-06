use crate::IdentMap;

pub trait TryIntoIdentMap<T>: Clone {
    fn try_into_identmap(&self) -> Option<&IdentMap<T>>;
}
