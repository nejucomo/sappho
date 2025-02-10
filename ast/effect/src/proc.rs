use sappho_unparse::{Stream, Unparse};

/// A proc effect can either be a mutation or a query effect.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProcEffect {
    /// Inquire is identical to [QueryEffect::Inquire](crate::QueryEffect::Inquire).
    Inquire,

    /// Evoke a mutation, as in `!exit`.
    Invoke,
}

impl Unparse for ProcEffect {
    fn unparse_into(&self, s: &mut Stream) {
        use ProcEffect::*;

        s.write(match self {
            Inquire => "$",
            Invoke => "!",
        });
    }
}
