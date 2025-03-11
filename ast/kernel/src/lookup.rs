use sappho_identifier::RcId;
use sappho_unparse::{Stream, Unparse};

use crate::{Expression, Recursion};

#[derive(Debug, derive_new::new)]
pub struct Lookup<X>
where
    X: Expression,
{
    #[new(into)]
    origin: Recursion<X>,
    attr: RcId,
}

impl<X> Unparse for Lookup<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.origin);
        s.write(".");
        s.write(&self.attr);
    }
}
