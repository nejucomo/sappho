use sappho_attrs::Attrs;
use sappho_identifier::RcId;

use crate::{AnnotationResult, ClosureInfo, IxRef};

#[derive(Debug, Default)]
pub struct ScopeInfoBuilder {
    /// Pending output
    sci: ClosureInfo,

    /// The map to indices of bindings declared in this scope
    declared: Attrs<IxRef>,
}

impl ScopeInfoBuilder {
    pub fn declare(&mut self, idref: RcId) -> AnnotationResult<()> {
        let ixref = IxRef::from(self.declared.len());
        self.declared.define(idref, ixref)?;
        Ok(())
    }

    pub fn idref_to_ixref(&mut self, outer: &ClosureInfo, idref: RcId) -> AnnotationResult<IxRef> {
        todo!()
    }
}
