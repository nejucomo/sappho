use std::ops::Index;

use crate::IxRef;

/// An index-based runtime local scope
///
/// # Design
///
/// [Scope] is designed to contain both bindings captured from an outer lexical scope as well as temporary local bindings.
///
/// This is parameterized primarily to facilitate crate separation. `Value` and [Scope] are mutually recursive, because objects capture [Scope].
#[derive(Debug)]
pub struct Scope<T>(Vec<T>);

impl<T> Scope<T> {
    /// Construct a new scope with sufficient capacity
    pub fn with_capacity(maxcap: usize) -> Self {
        Scope(Vec::with_capacity(maxcap))
    }

    /// Push a newly defined value
    pub fn bind(&mut self, value: T) {
        self.0.push(value);
    }
}

impl<T> Index<IxRef> for Scope<T> {
    type Output = T;

    fn index(&self, ix: IxRef) -> &Self::Output {
        &self.0[ix.0]
    }
}
