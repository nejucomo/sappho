use crate::{IxRef, ScopeInfoBuilder};

/// Runtime metadata for closures
///
/// # Design Notes
///
/// At runtime, the scope of references is a closure of bindings captured upon closure creation, along with a stack of local bindings.
///
/// When a closure is created, all non-local bindings are copied into the new closure, ensuring that every dereference is an efficient [IxRef] lookup.
#[derive(Debug, Default)]
pub struct ClosureInfo {
    /// A map for capturing references from the outer scope
    inheritance_map: Vec<IxRef>,

    /// The number of locally defined slots
    local_count: usize,
    // Do we need this for error message diagnostics?
    // ix2id: Vec<RcId>,
}

impl ClosureInfo {
    pub fn builder() -> ScopeInfoBuilder {
        ScopeInfoBuilder::default()
    }
}
