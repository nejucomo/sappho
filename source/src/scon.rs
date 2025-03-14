use derive_new::new;

#[derive(Debug, new)]
pub(crate) struct SourceG<P, S> {
    opath: Option<P>,
    text: S,
}
