use crate::IdentMap;

pub trait TryIntoIdentMap<T> {
    fn try_into_identmap(&self) -> Option<&IdentMap<T>>;
}
