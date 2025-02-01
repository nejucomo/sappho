use sappho_identmap::Identifier;
use sappho_legible::{IntoNode, KeyValue, Node};

#[derive(Debug)]
pub enum Element<F, Q, P, A> {
    Func(F),
    Query(Q),
    Proc(P),
    Attr(Identifier, A),
}

impl<'a, F, Q, P, A> IntoNode for Element<&'a F, &'a Q, &'a P, &'a A>
where
    &'a F: IntoNode,
    &'a Q: IntoNode,
    &'a P: IntoNode,
    &'a A: IntoNode,
{
    fn into_node(self) -> Node {
        use Element::*;

        match self {
            Func(f) => f.into_node(),
            Query(q) => q.into_node(),
            Proc(p) => p.into_node(),
            Attr(k, v) => KeyValue::new(k, v).into_node(),
        }
    }
}
