use sappho_unparse::{Stream, Unparse};

/// The query effect reads mutable memory.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QueryEffect {
    /// Inquire is the name of the `$myvar` effect syntax & semantics.
    Inquire,
}

impl Unparse for QueryEffect {
    fn unparse_into(&self, s: &mut Stream) {
        use QueryEffect::*;

        s.write(match self {
            Inquire => "$",
        });
    }
}
