use rand::distr::Distribution;
use sappho_source::{SourceCode, SourceCodeLink, SourceCodeRef};
use sappho_with_source::WithSource;

use crate::AstFuzz;

impl<T> Distribution<WithSource<T>> for AstFuzz
where
    AstFuzz: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> WithSource<T> {
        WithSource::new(
            rng.sample::<T, _>(self),
            SourceCodeRef::new(
                SourceCodeLink::from(SourceCode::new(
                    "<FAKE FUZZ CODE".to_string(),
                    "<FAKE FUZZ SOURCE>".to_string(),
                )),
                0..0,
            ),
        )
    }
}
