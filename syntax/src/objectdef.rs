use derive_more::From;
use sappho_ast_effect::Effect;
use sappho_attrs::Attrs;
use sappho_object::Object;

use crate::{FuncDef, ProcDef, QueryDef, Wise};

/// # TODO
///
/// Change the attributes to `Wise<PureEffect>` as a new restriction on object definitions.
#[derive(Debug, PartialEq, From)]
pub struct ObjectDef<FX>(Object<FuncDef, QueryDef, ProcDef, Wise<FX>>)
where
    FX: Effect;

impl<FX> ObjectDef<FX>
where
    FX: Effect,
{
    pub fn new<F, Q, P, A>(func: F, query: Q, proc: P, attrs: A) -> Self
    where
        Option<FuncDef>: From<F>,
        Option<QueryDef>: From<Q>,
        Option<ProcDef>: From<P>,
        Attrs<Wise<FX>>: From<A>,
    {
        ObjectDef(Object::new(
            Option::<FuncDef>::from(func),
            Option::<QueryDef>::from(query),
            Option::<ProcDef>::from(proc),
            Attrs::from(attrs),
        ))
    }
}

impl<FX> From<FuncDef> for ObjectDef<FX>
where
    FX: Effect,
{
    fn from(value: FuncDef) -> Self {
        Self::new(value, None, None, Attrs::default())
    }
}

impl<FX> From<QueryDef> for ObjectDef<FX>
where
    FX: Effect,
{
    fn from(value: QueryDef) -> Self {
        Self::new(None, value, None, Attrs::default())
    }
}

impl<FX> From<ProcDef> for ObjectDef<FX>
where
    FX: Effect,
{
    fn from(value: ProcDef) -> Self {
        Self::new(None, None, value, Attrs::default())
    }
}

impl<FX> From<Attrs<Wise<FX>>> for ObjectDef<FX>
where
    FX: Effect,
{
    fn from(value: Attrs<Wise<FX>>) -> Self {
        Self::new(None, None, None, value)
    }
}

mod parsing {
    use chumsky::Parser as _;
    use sappho_ast_effect::{Effect, ProcEffect, RestrictFrom, Restriction};
    use sappho_object::Object;
    use sappho_parsable::{ParsableWith, Parser};
    use sappho_unparse::{Stream, Unparse};

    use crate::parseparams::ParseParams;
    use crate::ObjectDef;

    impl<FX> ParsableWith<ParseParams<'_>> for ObjectDef<FX>
    where
        FX: Effect + RestrictFrom<ProcEffect>,
    {
        fn make_parser_with(pep: ParseParams<'_>) -> impl Parser<Self> {
            Object::make_parser_with(pep).map(Self)
        }
    }

    impl<FX> Unparse for ObjectDef<FX>
    where
        FX: Effect,
    {
        fn unparse_into(&self, s: &mut Stream) {
            self.0.unparse_into(s)
        }
    }

    impl<FX> RestrictFrom<ObjectDef<ProcEffect>> for ObjectDef<FX>
    where
        FX: Effect,
    {
        fn restrict(src: ObjectDef<ProcEffect>) -> Result<ObjectDef<FX>, Restriction> {
            Object::restrict(src.0).map(Self)
        }
    }
}
