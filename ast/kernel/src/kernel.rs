use crate::{Application, Confined, Root};

/// The common expression subset for all effects and richness-vs-reduction
#[derive(Debug, derive_more::From)]
pub enum Kernel<R>
where
    R: Root,
{
    Confined(Confined<R>),
    Application(Application<R>),
    // Let,
    // Lookup,
    // Match,
    // Object,
}
