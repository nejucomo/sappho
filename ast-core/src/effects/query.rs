use sappho_legible::{IntoNode, Node};

/// The query effect reads mutable memory.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QueryEffects {
    /// Inquire is the name of the `$myvar` effect syntax & semantics.
    Inquire,
}

impl QueryEffects {
    pub fn as_str(self) -> &'static str {
        use QueryEffects::*;

        match self {
            Inquire => "$",
        }
    }
}

impl IntoNode for QueryEffects {
    fn into_node(self) -> Node {
        self.as_str().into_node()
    }
}
