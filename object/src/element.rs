use sappho_attrs::Identifier;
use sappho_unparse::{Stream, Unparse};

#[derive(Debug)]
pub enum Element<F, Q, P, A> {
    Func(F),
    Query(Q),
    Proc(P),
    Attr(Identifier, A),
}

impl<'a, F, Q, P, A> Unparse for Element<&'a F, &'a Q, &'a P, &'a A>
where
    F: Unparse,
    Q: Unparse,
    P: Unparse,
    A: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use Element::*;

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
