use sappho_identifier::RcId;
use sappho_parsable::{ParsableWith, Parser, Recursive};
use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

use crate::r#let::Let;
use crate::r#match::Match;
use crate::{Application, Expression, Lookup, ObjectDef};

/// The common expression subset for all effects and richness-vs-reduction
#[derive(Debug, derive_more::From)]
pub enum Kernel<X>
where
    X: Expression,
{
    Prim(PrimVal),
    Ref(RcId),
    ObjectDef(ObjectDef<X>),
    Application(Application<X>),
    Lookup(Lookup<X>),
    Let(Let<X>),
    Match(Match<X>),
}

impl<X> ParsableWith<Recursive<'_, X>> for Kernel<X>
where
    X: Expression,
{
    fn make_parser_with(rec: Recursive<'_, X>) -> impl Parser<Self> {
        todo!()
    }
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
            ObjectDef(x) => x.unparse_into(s),
            Application(x) => x.unparse_into(s),
            Lookup(x) => x.unparse_into(s),
            Let(x) => x.unparse_into(s),
            Match(x) => x.unparse_into(s),
        }
    }
}
