use sappho_legible::{IntoNode, Node};

/// A literal value, such as `3.1415`.
#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Literal {
    /// A literal number value, such as `42`.
    Num(f64),
}

impl IntoNode for &Literal {
    fn into_node(self) -> Node {
        use Literal::*;

        match self {
            Num(x) => x.to_string().into_node(),
        }
    }
}
