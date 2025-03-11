use chumsky::prelude::just;
use chumsky::Parser as _;
use sappho_identifier::RcId;
use sappho_parsable::{Parsable, ParsableWith, Parser};
use sappho_unparse::{Stream, Unparse};

use self::Element::*;

#[derive(Debug)]
pub enum Element<F, Q, P, A> {
    Func(F),
    Query(Q),
    Proc(P),
    Attr(RcId, A),
}

impl<F, Q, P, A> Element<F, Q, P, A> {
    pub fn as_refs(&self) -> Element<&F, &Q, &P, &A> {
        match self {
            Func(f) => Func(f),
            Query(q) => Query(q),
            Proc(p) => Proc(p),
            Attr(id, a) => Attr(id.clone(), a),
        }
    }
}

impl<F, Q, P, A, ParseParam> ParsableWith<ParseParam> for Element<F, Q, P, A>
where
    ParseParam: Clone,
    F: ParsableWith<ParseParam>,
    Q: ParsableWith<ParseParam>,
    P: ParsableWith<ParseParam>,
    A: ParsableWith<ParseParam>,
{
    fn make_parser_with(param: ParseParam) -> impl Parser<Self> {
        F::parser_with(param.clone())
            .map(Func)
            .or(Q::parser_with(param.clone()).map(Query))
            .or(P::parser_with(param.clone()).map(Proc))
            .or(RcId::parser()
                .then_ignore(just(':').opt_space_around())
                .then(A::parser_with(param))
                .map(|(id, a)| Attr(id, a)))
    }
}

impl<F, Q, P, A> Unparse for Element<F, Q, P, A>
where
    F: Unparse,
    Q: Unparse,
    P: Unparse,
    A: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        match self {
            Func(f) => f.unparse_into(s),
            Query(q) => q.unparse_into(s),
            Proc(p) => p.unparse_into(s),
            Attr(k, v) => {
                s.write(k);
                s.write(": ");
                v.unparse_into(s);
            }
        }
    }
}
