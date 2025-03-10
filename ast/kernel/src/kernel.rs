use sappho_identifier::RcId;
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

use crate::r#let::Let;
use crate::r#match::Match;
use crate::{Application, Expression, ObjectDef};

/// The common expression subset for all effects and richness-vs-reduction
#[derive(Debug, derive_more::From)]
pub enum Kernel<X>
where
    X: Expression,
{
    Prim(PrimVal),
    Ref(RcId),
    Application(Application<X>),
    ObjectDef(ObjectDef<X>),
    Let(Let<X>),
    Match(Match<X>),
}

impl<X> Unparse for Kernel<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        use Kernel::*;

        match self {
            Prim(x) => x.unparse_into(s),
            Ref(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            ObjectDef(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
        }
    }
}
