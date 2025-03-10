use sappho_unparse::{Stream, Unparse};

use crate::Recursion;

#[derive(Debug, derive_new::new)]
pub struct Application<R>
where
    R: Unparse,
{
    target: Recursion<R>,
    argument: Recursion<R>,
}

impl<R> Unparse for Application<R>
where
    R: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.target);
        s.write(" ");
        s.write(&self.argument);
    }
}
