use crate::ScopeInfo;

/// Pre-runtime ScopeInfo for usize-ix-based scopes
#[derive(Debug, Default)]
pub struct LexStackInfoBuilder {
    /// The [ScopeInfo] for the innate scope
    innate: ScopeInfo,

    /// The lexical stack
    lexstack: Vec<ScopeInfo>,
}

impl LexStackInfoBuilder {
    /// Push a new [ScopeInfo]
    pub fn push(&mut self, sci: ScopeInfo) {
        self.lexstack.push(sci);
    }

    /// The inner-most lexical [ScopeInfo]
    ///
    /// For the initial expression, this is for the innate scope.
    pub fn innermost(&mut self) -> &mut ScopeInfo {
        self.lexstack.last_mut().unwrap_or(&mut self.innate)
    }
}
