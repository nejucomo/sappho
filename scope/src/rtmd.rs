use crate::{IxRef, Scope};

/// Static runtime metadata about lexical [Scope] and closure captures
#[derive(Debug)]
pub struct RtMetadata {
    /// Total stack size
    ///
    /// This is the number of slots to allocate to ensure all captures and lexical locals can be stored.
    maxcap: usize,

    /// The mapping of captures from an outer scope to a newly constructed scope
    inheritance_map: Vec<IxRef>,
}

impl RtMetadata {
    /// Initialize a new inner [Scope] from the outer [Scope]
    pub fn init_scope<T>(&self, outer: &Scope<T>) -> Scope<T>
    where
        T: Clone,
    {
        let mut inner = Scope::with_capacity(self.maxcap);
        for ix in self.inheritance_indices() {
            inner.bind(outer[ix].clone());
        }
        inner
    }

    fn inheritance_indices(&self) -> impl Iterator<Item = IxRef> + '_ {
        self.inheritance_map.iter().copied()
    }
}
