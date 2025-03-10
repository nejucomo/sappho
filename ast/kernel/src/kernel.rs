use sappho_unparse::{Stream, Unparse};

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

impl<R> Unparse for Kernel<R>
where
    R: Root,
{
    fn unparse_into(&self, s: &mut Stream) {
        use Kernel::*;

        match self {
            Confined(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
        }
    }
}
