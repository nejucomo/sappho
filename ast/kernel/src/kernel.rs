use sappho_identifier::RcId;
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

use crate::{Application, ObjectDef};

/// The common expression subset for all effects and richness-vs-reduction
#[derive(Debug, derive_more::From)]
pub enum Kernel<R>
where
    R: Unparse,
{
    Prim(PrimVal),
    Ref(RcId),
    Application(Application<R>),
    ObjectDef(ObjectDef<R>),
}

impl<R> Unparse for Kernel<R>
where
    R: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use Kernel::*;

        match self {
            Prim(x) => x.unparse_into(s),
            Ref(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            ObjectDef(x) => x.unparse_into(s),
        }
    }
}
