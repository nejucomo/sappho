use rand::distr::Distribution;
use rand::Rng;
use sappho_ast::ProcEffects;

use crate::AstFuzz;

impl Distribution<ProcEffects> for AstFuzz {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ProcEffects {
        let _ = rng;
        todo!()
        // self.weighted_case(3, || ProcEffects::Inquire)
        //     .or(weighted_case(
    }
}
