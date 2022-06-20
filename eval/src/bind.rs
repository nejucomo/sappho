use crate::scope::ScopeRef;
use crate::ValRef;
use sappho_east::Pattern;

/// Attempt to bind `value` to `pattern` and on success return a new scope. A `None` result means
/// the pattern does not match.
pub(crate) fn bind(pattern: &Pattern, value: &ValRef, scope: ScopeRef) -> Option<ScopeRef> {
    Some(scope.extend(pattern, value.clone()))
}
