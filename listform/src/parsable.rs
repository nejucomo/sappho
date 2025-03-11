use std::fmt::Debug;

use chumsky::prelude::just;
use chumsky::Parser as _;
use either::Either::{Left, Right};
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use crate::ListForm;

impl<X, PX, T, PT> ParsableWith<(PX, PT)> for ListForm<X, T>
where
    X: Unparse + Debug,
    T: Unparse + Debug,
    PX: Parser<X>,
    PT: Parser<T>,
{
    fn make_parser_with((item, tail): (PX, PT)) -> impl Parser<Self> {
        let tailmatch = || just("..").ignore_then(tail.clone());
        let nonempty_body = item.separated_by(just(',').then_opt_space());

        let nonempty_opt_tail = nonempty_body
            .then(just(',').then_opt_space().ignore_then(tailmatch()).or_not())
            .map(|(pats, opttail)| ListForm::new(pats, opttail));

        bracketed(
            ['[', ']'],
            tailmatch()
                .map(|t| ListForm::new([], Some(t)))
                .or(nonempty_opt_tail)
                .or_not()
                .map(|opt| opt.unwrap_or_else(|| ListForm::new([], None))),
        )
    }
}

impl<X, T> Unparse for ListForm<X, T>
where
    X: Unparse,
    T: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Brackets::Square;
        use sappho_unparse::Break::OptSpace;

        if self.is_empty() {
            s.write("[]")
        } else {
            s.bracketed(Square, |subs| {
                let mut first = true;

                for xort in self.lfg_ref() {
                    match xort {
                        Left(elem) => {
                            if first {
                                first = false;
                            } else {
                                subs.write(",");
                            }
                            subs.write(&OptSpace);
                            subs.write(elem);
                        }
                        Right(tail) => {
                            if !first {
                                subs.write(",");
                            }
                            subs.write(&OptSpace);
                            subs.write("..");
                            subs.write(tail);
                        }
                    }
                }
            });
        }
    }
}
