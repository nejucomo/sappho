use sappho_legible::{BracketSeq, IntoNode, Node};

use crate::Statements;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub struct ProcDef<ProcExpr>(Statements<ProcExpr>);

impl<X> ProcDef<X> {
    pub fn transform_into<XD>(self) -> ProcDef<XD>
    where
        XD: From<X>,
    {
        ProcDef(self.0.transform_into())
    }
}

impl<'a, X> IntoNode for &'a ProcDef<X>
where
    &'a X: IntoNode,
{
    fn into_node(self) -> Node {
        BracketSeq::new(("proc {", "}"), ";", self.0.into_node_iter()).into_node()
    }
}
