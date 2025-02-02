use sappho_legible::{IntoNode, Node};

/// Pure effects cannot be instantiated, because pure expressions have no side effects.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PureEffects {}

impl IntoNode for &PureEffects {
    fn into_node(self) -> Node {
        unreachable!("pure effects are never instantiated");
    }
}
