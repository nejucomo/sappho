use sappho_primval::PrimVal;
use sappho_unparse::{Stream, Unparse};

/// A literal value, such as `3.1415`.
///
/// # TODO
///
/// Replace directly with [PrimVal]?
#[derive(Copy, Clone, Debug, PartialEq, derive_more::From)]
pub enum Literal {
    /// A literal number value, such as `42`.
    Num(f64),
}

impl From<PrimVal> for Literal {
    fn from(pv: PrimVal) -> Self {
        match pv {
            PrimVal::Num(n) => Self::Num(n),
        }
    }
}

impl Unparse for Literal {
    fn unparse_into(&self, s: &mut Stream) {
        use Literal::*;

        match self {
            Num(x) => s.write(&x.to_string()),
        }
    }
}
