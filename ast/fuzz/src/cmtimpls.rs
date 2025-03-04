use rand::distr::Distribution;
use sappho_ast_comment::CmtPrefixed;

use crate::AstFuzz;

impl<T> Distribution<CmtPrefixed<T>> for AstFuzz
where
    AstFuzz: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CmtPrefixed<T> {
        CmtPrefixed::new(Some(format!("generated from {self:?}")), self.sample(rng))
    }
}
