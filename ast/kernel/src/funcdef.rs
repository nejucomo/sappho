use sappho_unparse::{Stream, Unparse};

use crate::{Expression, Pattern, Recursion};

#[derive(Debug, derive_new::new)]
pub struct FuncDef<X>
where
    X: Expression,
{
    binding: Pattern<X>,
    body: Recursion<X>,
}

impl<X> Unparse for FuncDef<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("fn ");
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.body);
    }
}
