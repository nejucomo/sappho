use crate::ScopeInfo;

/// ScopeInfo-Annotated-Node
#[derive(Debug)]
pub struct ScopeAnnotated<T> {
    node: T,
    scopeinfo: ScopeInfo,
}
