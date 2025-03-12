use sappho_unparse::{Stream, Unparse};

use crate::error::Span;

#[derive(Debug, derive_new::new)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Unparse for Spanned<T>
where
    T: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        self.node.unparse_into(s)
    }
}
