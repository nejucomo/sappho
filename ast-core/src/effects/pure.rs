use sappho_legible::{IntoNode, Node};

/// Pure effects cannot be instantiated, because pure expressions have no side effects.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PureEffects {}

impl<'a> IntoNode for &'a PureEffects {
    fn into_node(self) -> Node {
        unreachable!("pure effects are never instantiated");
    }
}
