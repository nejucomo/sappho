use sappho_unparse::Unparse;

use crate::{Expression, Pattern, Recursion};

/// # TODO
///
/// Change arrow to `=>` distinct from `fn`
#[derive(Debug, derive_new::new)]
pub struct MatchClause<X>
where
    X: Expression,
{
    binding: Pattern<X>,
    consequent: Recursion<X>,
}

impl<X> Unparse for MatchClause<X>
where
    X: Expression,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        s.write(&self.binding);
        s.write(" -> ");
        s.write(&self.consequent);
    }
}
