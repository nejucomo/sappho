use crate::ValRef;
use sappho_identmap::IdentMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, derive_more::From)]
pub struct Frame(IdentMap<ValRef>);

impl Deref for Frame {
    type Target = IdentMap<ValRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Frame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
