use crate::{AnnotationResult, LexStackInfoBuilder};

pub trait AnnotateScope: Sized {
    type BeastNode;

    fn annotate_scope(self) -> AnnotationResult<Self::BeastNode> {
        let mut lsib = LexStackInfoBuilder::default();
        self.asi_recurse(&mut lsib)
    }

    #[doc(hidden)]
    fn asi_recurse(self, lsib: &mut LexStackInfoBuilder) -> AnnotationResult<Self::BeastNode>;
}
