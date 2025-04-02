use std::fmt::Debug;

use chumsky::prelude::just;
use chumsky::Parser as _;
use derive_new::new;

use sappho_attrs::Attrs;
use sappho_effect::{RestrictFrom, Restriction};
use sappho_parsable::primitive::bracketed;
use sappho_parsable::{ParsableWith, Parser};
use sappho_tfi::TryFromIterator;
use sappho_unparse::{Stream, Unparse};

use crate::{Element, IntoIter, Unbundled};

#[derive(Clone, Debug, PartialEq, new)]
pub struct Object<F, Q, P, A>
where
    A: Debug,
{
    #[new(into)]
    f: Option<F>,
    #[new(into)]
    q: Option<Q>,
    #[new(into)]
    p: Option<P>,
    #[new(into)]
    a: Attrs<A>,
}

impl<F, Q, P, A> Default for Object<F, Q, P, A>
where
    A: Debug,
{
    fn default() -> Self {
        Object::new(None, None, None, Attrs::default())
    }
}

impl<F, Q, P, A> Object<F, Q, P, A>
where
    A: Debug,
{
    pub fn new_func(func: F) -> Self {
        Self::new(Some(func), None, None, Attrs::default())
    }

    pub fn new_query(query: Q) -> Self {
        Self::new(None, Some(query), None, Attrs::default())
    }

    pub fn new_proc(proc: P) -> Self {
        Self::new(None, None, Some(proc), Attrs::default())
    }

    pub fn new_attrs<T>(attrs: T) -> Self
    where
        T: Into<Attrs<A>>,
    {
        Self::new(None, None, None, attrs.into())
    }

    pub fn func(&self) -> Option<&F> {
        self.f.as_ref()
    }

    pub fn query(&self) -> Option<&Q> {
        self.q.as_ref()
    }

    pub fn proc(&self) -> Option<&P> {
        self.p.as_ref()
    }

    pub fn attrs(&self) -> &Attrs<A> {
        &self.a
    }

    pub fn unbundle(self) -> Unbundled<F, Q, P, A> {
        use Unbundled::*;

        match self {
            Object {
                f: None,
                q: None,
                p: None,
                a,
            } => Attrs(a),

            Object {
                f: Some(f),
                q: None,
                p: None,
                a,
            } if a.is_empty() => Func(f),

            Object {
                f: None,
                q: Some(q),
                p: None,
                a,
            } if a.is_empty() => Query(q),

            Object {
                f: None,
                q: None,
                p: Some(p),
                a,
            } if a.is_empty() => Proc(p),

            bundle => Bundled(bundle),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.f.is_none() && self.q.is_none() && self.p.is_none() && self.a.is_empty()
    }

    pub fn map_parts<TF, FR, TQ, QR, TP, PR, TA, AR>(
        self,
        tfunc: TF,
        tquery: TQ,
        tproc: TP,
        tattr: TA,
    ) -> Object<FR, QR, PR, AR>
    where
        AR: Debug,
        TF: FnOnce(F) -> FR,
        TQ: FnOnce(Q) -> QR,
        TP: FnOnce(P) -> PR,
        TA: Fn(A) -> AR,
    {
        Object {
            f: self.f.map(tfunc),
            q: self.q.map(tquery),
            p: self.p.map(tproc),
            a: self.a.map(tattr),
        }
    }

    pub fn into_try_map_values<TA, DA, E>(self, tattr: TA) -> Result<Object<F, Q, P, DA>, E>
    where
        DA: Debug,
        TA: Fn(A) -> Result<DA, E>,
    {
        let mut dsta = Attrs::default();
        for (aname, x) in self.a {
            let dx = tattr(x)?;
            dsta.define(aname, dx).unwrap();
        }
        Ok(Object::new(self.f, self.q, self.p, dsta))
    }

    pub fn as_refs(&self) -> Object<&F, &Q, &P, &A> {
        Object {
            f: self.f.as_ref(),
            q: self.q.as_ref(),
            p: self.p.as_ref(),
            a: self.a.iter().map(|(k, v)| (k.clone(), v)).collect(),
        }
    }
}

impl<F, Q, P, A> IntoIterator for Object<F, Q, P, A>
where
    A: Debug,
{
    type Item = Element<F, Q, P, A>;
    type IntoIter = IntoIter<F, Q, P, A>;

    fn into_iter(self) -> Self::IntoIter {
        let Object { f, q, p, a } = self;
        IntoIter {
            f,
            q,
            p,
            a: a.into_iter(),
        }
    }
}

impl<F, Q, P, A> TryFromIterator<Element<F, Q, P, A>> for Object<F, Q, P, A>
where
    A: Debug,
{
    type Error = String;

    fn try_append(mut self, elem: Element<F, Q, P, A>) -> Result<Self, Self::Error> {
        use Element::*;

        fn set_up_to_one<T>(
            pluralname: &'static str,
            opt: &mut Option<T>,
            val: T,
        ) -> Result<(), String> {
            if opt.replace(val).is_some() {
                Err(format!(
                    "multiple {pluralname} disallowed in object creation"
                ))
            } else {
                Ok(())
            }
        }

        match elem {
            Func(f) => set_up_to_one("funcs", &mut self.f, f).map(|()| self),
            Query(q) => set_up_to_one("queries", &mut self.q, q).map(|()| self),
            Proc(p) => set_up_to_one("procs", &mut self.p, p).map(|()| self),
            Attr(k, v) => self
                .a
                .define(k.clone(), v)
                .map(|()| self)
                .map_err(|_| format!("duplicate attribute {:?}", k.as_str())),
        }
    }
}

impl<F, Q, P, A> FromIterator<Element<F, Q, P, A>> for Result<Object<F, Q, P, A>, String>
where
    A: Debug,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Element<F, Q, P, A>>,
    {
        let mut obj = Object::default();
        for elem in iter {
            obj = obj.try_append(elem)?;
        }
        Ok(obj)
    }
}

impl<F, Q, P, A, ParseParam> ParsableWith<ParseParam> for Object<F, Q, P, A>
where
    ParseParam: Clone,
    F: ParsableWith<ParseParam>,
    Q: ParsableWith<ParseParam>,
    P: ParsableWith<ParseParam>,
    A: ParsableWith<ParseParam> + Debug,
{
    fn make_parser_with(param: ParseParam) -> impl Parser<Self> {
        bracketed(
            ['{', '}'],
            Element::parser_with(param)
                .separated_by(just(',').opt_space_around())
                .try_map_ez(Object::try_from_iterator),
        )
    }
}

impl<F, Q, P, A> Unparse for Object<F, Q, P, A>
where
    F: Unparse,
    Q: Unparse,
    P: Unparse,
    A: Unparse + Debug,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Brackets::Squiggle;
        use sappho_unparse::Break::OptSpace;

        if self.is_empty() {
            s.write("{}");
        } else {
            s.bracketed(Squiggle, |subs| {
                for elem in self.as_refs().into_iter() {
                    use Element::*;

                    subs.write(&OptSpace);
                    // HACK BUG TODO: Explore a pivot to `Unparse::unparse_into(self, ...)` not `&self`
                    // subs.write(&elem);
                    match elem {
                        Func(f) => f.unparse_into(subs),
                        Query(q) => q.unparse_into(subs),
                        Proc(p) => p.unparse_into(subs),
                        Attr(k, v) => {
                            subs.write(&k);
                            subs.write(": ");
                            v.unparse_into(subs);
                        }
                    }
                    subs.write(",");
                }
            });
        }
    }
}

impl<F, Q, P, AS, AT> RestrictFrom<Object<F, Q, P, AS>> for Object<F, Q, P, AT>
where
    AS: Debug,
    AT: RestrictFrom<AS> + Debug,
{
    fn restrict(src: Object<F, Q, P, AS>) -> Result<Self, Restriction> {
        src.into_try_map_values(|asrc| AT::restrict(asrc))
    }
}
