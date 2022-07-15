use sappho_unparse::{Stream, Unparse};

/// The query effect reads mutable memory.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QueryEffects {
    /// Inquire is the name of the `$myvar` effect syntax & semantics.
    Inquire,
}

impl Unparse for QueryEffects {
    fn unparse_into(&self, s: &mut Stream) {
        use QueryEffects::*;

        s.write(match self {
            Inquire => "$",
        });
    }
}
