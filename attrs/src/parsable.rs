use std::fmt::Debug;

use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_identifier::RcId;
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_tfi::TryFromIterator;
use sappho_unparse::Unparse;

use crate::Attrs;

impl<P, T> ParsableWith<P> for Attrs<T>
where
    P: Parser<T>,
    T: Unparse + Debug,
{
    fn make_parser_with(attr: P) -> impl Parser<Self> {
        bracketed(
            ['{', '}'],
            RcId::parser()
                .then_ignore(just(':').opt_space_around())
                .then(attr)
                .separated_by(just(',').then_opt_space())
                .allow_trailing(),
        )
        .try_map_ez(Attrs::try_from_iterator)
    }
}

impl<T> Unparse for Attrs<T>
where
    T: Unparse + Debug,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        use sappho_unparse::{Brackets::Squiggle, Break::OptSpace};

        if self.is_empty() {
            s.write("{}");
        } else {
            s.bracketed(Squiggle, |subs| {
                for (k, v) in self.iter() {
                    subs.write(&OptSpace);
                    subs.write(k);
                    subs.write(": ");
                    subs.write(v);
                    subs.write(",");
                }
            });
        }
    }
}
