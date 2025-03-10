use sappho_unparse::{Stream, Unparse};

use crate::{Kernel, Root};

/// A [QueryX] is an expression with query effects
#[derive(Debug, derive_more::From)]
pub enum QueryX {
    Kernel(Kernel<QueryX>),
    Inquire(Box<QueryX>),
}

impl Root for QueryX {}

impl Unparse for QueryX {
    fn unparse_into(&self, s: &mut Stream) {
        use QueryX::*;

        match self {
            Kernel(x) => x.unparse_into(s),
            Inquire(x) => x.unparse_into(s),
        }
    }
}
