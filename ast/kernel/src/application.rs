use crate::{Confined, Root};

#[derive(Debug, derive_new::new)]
pub struct Application<R>
where
    R: Root,
{
    target: Confined<R>,
    argument: Confined<R>,
}
