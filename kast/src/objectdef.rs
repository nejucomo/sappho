use derive_more::From;
use sappho_attrs::Attrs;
use sappho_effect::Effect;
use sappho_object::Object;

use crate::{FuncDef, KastProvider, ProcDef, QueryDef, Wise};

/// # TODO
///
/// Change the attributes to `Wise<PureEffect>` as a new restriction on object definitions.
#[derive(Clone, Debug, PartialEq, From)]
pub struct ObjectDef<K, FX>(Object<FuncDef<K>, QueryDef<K>, ProcDef<K>, Wise<K, FX>>)
where
    K: KastProvider,
    FX: Effect;

impl<K, FX> ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    pub fn new<F, Q, P, A>(func: F, query: Q, proc: P, attrs: A) -> Self
    where
        Option<FuncDef<K>>: From<F>,
        Option<QueryDef<K>>: From<Q>,
        Option<ProcDef<K>>: From<P>,
        Attrs<Wise<K, FX>>: From<A>,
    {
        ObjectDef(Object::new(
            Option::<FuncDef<K>>::from(func),
            Option::<QueryDef<K>>::from(query),
            Option::<ProcDef<K>>::from(proc),
            Attrs::from(attrs),
        ))
    }
}

impl<K, FX> From<FuncDef<K>> for ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: FuncDef<K>) -> Self {
        Self::new(value, None, None, Attrs::default())
    }
}

impl<K, FX> From<QueryDef<K>> for ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: QueryDef<K>) -> Self {
        Self::new(None, value, None, Attrs::default())
    }
}

impl<K, FX> From<ProcDef<K>> for ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: ProcDef<K>) -> Self {
        Self::new(None, None, value, Attrs::default())
    }
}

impl<K, FX> From<Attrs<Wise<K, FX>>> for ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: Attrs<Wise<K, FX>>) -> Self {
        Self::new(None, None, None, value)
    }
}

mod parsing {
    use chumsky::Parser as _;
    use sappho_effect::{Effect, ProcEffect, PureEffect, QueryEffect, RestrictFrom, Restriction};
    use sappho_object::Object;
    use sappho_parsable::{ParsableWith, Parser};
    use sappho_unparse::{Stream, Unparse};

    use crate::{FuncDef, KastProvider, ObjectDef, ProcDef, ProcWiseParser, QueryDef};

    impl<'a, K, FX> ParsableWith<ProcWiseParser<'a, K>> for ObjectDef<K, FX>
    where
        K: KastProvider,
        FX: Effect,
        K::Expr<PureEffect>:
            ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<QueryEffect>:
            ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<ProcEffect>:
            ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<FX>: ParsableWith<ProcWiseParser<'a, K>> + RestrictFrom<K::Expr<ProcEffect>>,
    {
        fn make_parser_with(pep: ProcWiseParser<'a, K>) -> impl Parser<Self> {
            Object::make_parser_with(pep).map(Self)
        }
    }

    impl<K, FX> Unparse for ObjectDef<K, FX>
    where
        K: KastProvider,
        FX: Effect,
        FuncDef<K>: Unparse,
        QueryDef<K>: Unparse,
        ProcDef<K>: Unparse,
        K::Expr<FX>: Unparse,
    {
        fn unparse_into(&self, s: &mut Stream) {
            self.0.unparse_into(s)
        }
    }

    impl<K, FX> RestrictFrom<ObjectDef<K, ProcEffect>> for ObjectDef<K, FX>
    where
        K: KastProvider,
        FX: Effect,
        K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
    {
        fn restrict(src: ObjectDef<K, ProcEffect>) -> Result<ObjectDef<K, FX>, Restriction> {
            Object::restrict(src.0).map(Self)
        }
    }
}
