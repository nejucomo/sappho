use sappho_source::SourceCodeRef;
use sappho_with_source::WithSource;

pub(crate) trait TransformWithSource<T> {
    fn transform_with_source(self, src: Option<SourceCodeRef>) -> WithSource<T>;
}
