use derive_new::new;

use crate::WithAuxillary;

#[derive(Debug, new)]
pub struct ContinueStep<X, C> {
    pub expr: X,
    pub cont: C,
}

impl<X, C> ContinueStep<X, C> {
    pub fn from_iter(itercont: C, init: I) -> ContinueStep<X, 

    pub fn with_aux<A>(self, aux: A) -> ContinueStep<X, WithAuxillary<C, A>> {
        ContinueStep {
            expr: self.expr,
            cont: WithAuxillary::new(self.cont, aux),
        }
    }
}

impl<A, X, C> ContinueStep<(A, X), C> {
    pub fn factor_auxillary(self) -> ContinueStep<X, WithAuxillary<C, A>> {
        let ContinueStep {
            expr: (aux, expr),
            cont,
        } = self;

        ContinueStep { expr, cont }.with_aux(aux)
    }
}
