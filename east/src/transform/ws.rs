use sappho_source::SourceCodeRef;

pub(crate) trait TransformWithSource<T> {
    fn transform_with_source(self, src: &Option<SourceCodeRef>) -> T;
}

impl<S, T> TransformWithSource<Vec<T>> for Vec<S>
where
    S: TransformWithSource<T>,
{
    fn transform_with_source(self, src: &Option<SourceCodeRef>) -> Vec<T> {
        self.into_iter()
            .map(|s| s.transform_with_source(src))
            .collect()
    }
}
