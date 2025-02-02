use sappho_legible::{IntoNode, Node};

/// A proc effect can either be a mutation or a query effect.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProcEffects {
    /// Inquire is identical to [QueryEffects::Inquire](crate::QueryEffects::Inquire).
    Inquire,

    /// Evoke a mutation, as in `!exit`.
    Invoke,
}

impl ProcEffects {
    pub fn as_str(self) -> &'static str {
        use ProcEffects::*;

        match self {
            Inquire => "$",
            Invoke => "!",
        }
    }
}

impl IntoNode for &ProcEffects {
    fn into_node(self) -> Node {
        self.as_str().into_node()
    }
}
