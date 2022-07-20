use sappho_unparse::{Stream, Unparse};

/// A proc effect can either be a mutation or a query effect.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProcEffects {
    /// Inquire is identical to [QueryEffects::Inquire](crate::QueryEffects::Inquire).
    Inquire,

    /// Evoke a mutation, as in `!exit`.
    Invoke,
}

impl Unparse for ProcEffects {
    fn unparse_into(&self, s: &mut Stream) {
        use ProcEffects::*;

        s.write(match self {
            Inquire => "$",
            Invoke => "!",
        });
    }
}
