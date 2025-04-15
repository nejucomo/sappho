use std::rc::Rc;

use derive_more::{Deref, From, Into};
use sappho_attrs::Attrs;
use sappho_effect::Effect;
use sappho_object::Object;

use crate::{FuncDef, KastProvider, ProcDef, QueryDef, Wise};

/// # TODO
///
/// Change the attributes to `Wise<PureEffect>` as a new restriction on object definitions.
#[derive(Clone, Debug, PartialEq, From, Into, Deref)]
pub struct ObjectDef<K, FX>(ObjectDefInner<K, FX>)
where
    K: KastProvider,
    FX: Effect;

pub type ObjectDefInner<K, FX> =
    Object<Rc<FuncDef<K>>, Rc<QueryDef<K>>, Rc<ProcDef<K>>, Wise<K, FX>>;

impl<K, FX> ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    pub fn new<F, Q, P, A>(func: F, query: Q, proc: P, attrs: A) -> Self
    where
        F: Into<Option<Rc<FuncDef<K>>>>,
        Q: Into<Option<Rc<QueryDef<K>>>>,
        P: Into<Option<Rc<ProcDef<K>>>>,
        A: Into<Attrs<Wise<K, FX>>>,
    {
        ObjectDef(Object::new(
            func.into(),
            query.into(),
            proc.into(),
            attrs.into(),
        ))
    }

    pub fn unwrap(self) -> ObjectDefInner<K, FX> {
        self.0
    }
}

impl<K, FX> From<Rc<FuncDef<K>>> for ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: Rc<FuncDef<K>>) -> Self {
        Self::new(value, None, None, Attrs::default())
    }
}

impl<K, FX> From<Rc<QueryDef<K>>> for ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: Rc<QueryDef<K>>) -> Self {
        Self::new(None, value, None, Attrs::default())
    }
}

impl<K, FX> From<Rc<ProcDef<K>>> for ObjectDef<K, FX>
where
    K: KastProvider,
    FX: Effect,
{
    fn from(value: Rc<ProcDef<K>>) -> Self {
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

    use crate::{KastProvider, ObjectDef, ProcWiseParser};

    impl<K, FX> ParsableWith<ProcWiseParser<'_, K>> for ObjectDef<K, FX>
    where
        K: KastProvider,
        K::Expr<FX>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<PureEffect>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<QueryEffect>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<ProcEffect>: Unparse + RestrictFrom<K::Expr<ProcEffect>>,
        FX: Effect,
    {
        fn make_parser_with(pep: ProcWiseParser<'_, K>) -> impl Parser<Self> {
            Object::make_parser_with(pep).map(Self)
        }
    }

    impl<K, FX> Unparse for ObjectDef<K, FX>
    where
        K: KastProvider,
        K::Expr<FX>: Unparse,
        K::Expr<PureEffect>: Unparse,
        K::Expr<QueryEffect>: Unparse,
        K::Expr<ProcEffect>: Unparse,
        FX: Effect,
    {
        fn unparse_into(&self, s: &mut Stream) {
            self.0.unparse_into(s)
        }
    }

    impl<K, FX> RestrictFrom<ObjectDef<K, ProcEffect>> for ObjectDef<K, FX>
    where
        K: KastProvider,
        K::Expr<FX>: RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<PureEffect>: RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<QueryEffect>: RestrictFrom<K::Expr<ProcEffect>>,
        K::Expr<ProcEffect>: RestrictFrom<K::Expr<ProcEffect>>,
        FX: Effect,
    {
        fn restrict(src: ObjectDef<K, ProcEffect>) -> Result<ObjectDef<K, FX>, Restriction> {
            Object::restrict(src.0).map(Self)
        }
    }
}
