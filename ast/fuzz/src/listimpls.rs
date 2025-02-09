use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::{Expr, ListExpr};
use sappho_listform::ListForm;

use crate::AstFuzz;

impl<FX> Distribution<ListExpr<FX>> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ListExpr<FX> {
        let mut body = vec![];
        while rng.random_ratio(2, 3) {
            body.push(rng.sample(self));
        }
        let optail = rng.sample::<Option<Expr<FX>>, _>(self).map(Box::new);
        ListForm::new(body, optail)
    }
}
