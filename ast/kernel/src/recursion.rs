use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_more::From)]
#[from(R)]
pub struct Recursion<R>(Box<R>)
where
    R: Unparse;

impl<R> Unparse for Recursion<R>
where
    R: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.0.unparse_into(s)
    }
}
