use sappho_unparse::{Stream, Unparse};

use crate::{Confined, Root};

#[derive(Debug, derive_new::new)]
pub struct Application<R>
where
    R: Root,
{
    target: Confined<R>,
    argument: Confined<R>,
}

impl<R> Unparse for Application<R>
where
    R: Root,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.target);
        s.write(" ");
        s.write(&self.argument);
    }
}
