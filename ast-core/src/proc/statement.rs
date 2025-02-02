use sappho_legible::{IntoNode, Node};

#[derive(Clone, Debug, PartialEq)]
pub enum Statements<ProcExpr> {
    Return(Box<ProcExpr>),
}

impl<X> Statements<X> {
    pub fn transform_into<XD>(self) -> Statements<XD>
    where
        XD: From<X>,
    {
        use Statements::*;

        match self {
            Return(x) => Return(Box::new(XD::from(*x))),
        }
    }

    pub(crate) fn node_iter<'a>(&'a self) -> impl Iterator<Item = Node>
    where
        &'a X: IntoNode,
    {
        use Statements::*;

        match self {
            Return(x) => Some(("return ", x, ";").into_node()).into_iter(),
        }
    }
}
