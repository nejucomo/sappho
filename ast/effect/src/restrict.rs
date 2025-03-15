use crate::{Effect, EffectDescription, ProcEffect, PureEffect, QueryEffect};

/// Restrict from more permissive effects to more restricted effects
pub trait RestrictFrom<S>: Sized {
    /// Attempt to restrict from `src`, or `None` for invalid restrictions
    fn restrict(src: S) -> Result<Self, Restriction>;
}

#[derive(Debug, thiserror::Error)]
#[error("failed to restrict {sourcefx} in {targetctx} context")]
pub struct Restriction {
    sourcefx: String,
    targetctx: &'static str,
}

impl Restriction {
    pub(crate) fn new<FXT, FXS>(source_effect: FXS) -> Self
    where
        FXT: Effect,
        FXS: Effect,
    {
        let EffectDescription { noun, sigil, .. } = source_effect.description();

        Restriction {
            sourcefx: format!("{noun} (ie: `{sigil}x`)"),
            targetctx: FXT::context(),
        }
    }
}

// The base restrictions are:
impl RestrictFrom<ProcEffect> for PureEffect {
    fn restrict(pfx: ProcEffect) -> Result<Self, Restriction> {
        Err(Restriction::new::<Self, _>(pfx))
    }
}

impl RestrictFrom<QueryEffect> for PureEffect {
    fn restrict(qfx: QueryEffect) -> Result<Self, Restriction> {
        Err(Restriction::new::<Self, _>(qfx))
    }
}

impl RestrictFrom<PureEffect> for PureEffect {
    fn restrict(pure: PureEffect) -> Result<Self, Restriction> {
        Ok(pure)
    }
}

impl RestrictFrom<ProcEffect> for QueryEffect {
    fn restrict(pfx: ProcEffect) -> Result<Self, Restriction> {
        match pfx {
            ProcEffect::Inquire => Ok(QueryEffect::Inquire),
            pfx => Err(Restriction::new::<Self, _>(pfx)),
        }
    }
}

impl RestrictFrom<QueryEffect> for QueryEffect {
    fn restrict(qfx: QueryEffect) -> Result<Self, Restriction> {
        Ok(qfx)
    }
}

impl RestrictFrom<ProcEffect> for ProcEffect {
    fn restrict(pfx: ProcEffect) -> Result<Self, Restriction> {
        Ok(pfx)
    }
}

// Generic container impls:
impl<T, S> RestrictFrom<Box<S>> for Box<T>
where
    T: RestrictFrom<S>,
{
    fn restrict(src: Box<S>) -> Result<Box<T>, Restriction> {
        T::restrict(*src).map(Box::new)
    }
}
