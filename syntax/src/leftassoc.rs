use chumsky::Parser as _;
use derive_more::From;
use sappho_effect::{RestrictFrom, Restriction};
use sappho_parsable::{ParsableWith, Parser};
use sappho_unparse::Unparse;

#[derive(Clone, Debug, PartialEq, From)]
pub struct LeftAssoc<L, R> {
    left: L,
    rights: Vec<R>,
}

impl<L, R> LeftAssoc<L, R> {
    pub fn new<LS, RSI, RS>(left: LS, rights: RSI) -> Self
    where
        L: From<LS>,
        R: From<RS>,
        RSI: IntoIterator<Item = RS>,
    {
        LeftAssoc {
            left: L::from(left),
            rights: rights.into_iter().map(R::from).collect(),
        }
    }

    pub fn ref_left(&self) -> &L {
        &self.left
    }

    pub fn is_just_left(&self) -> bool {
        self.rights.is_empty()
    }

    pub fn map_left<ML, L2>(self, map_left: ML) -> LeftAssoc<L2, R>
    where
        ML: FnOnce(L) -> L2,
    {
        LeftAssoc {
            left: map_left(self.left),
            rights: self.rights,
        }
    }

    pub fn try_map<ML, MR, L2, R2, E>(
        self,
        map_left: ML,
        map_right: MR,
    ) -> Result<LeftAssoc<L2, R2>, E>
    where
        ML: FnOnce(L) -> Result<L2, E>,
        MR: Fn(R) -> Result<R2, E>,
    {
        let left = map_left(self.left)?;
        let rights = self
            .rights
            .into_iter()
            .map(map_right)
            .collect::<Result<_, _>>()?;

        Ok(LeftAssoc { left, rights })
    }
}

mod fromimpls {
    use sappho_effect::Effect;

    use crate::leftassoc::LeftAssoc;
    use crate::{Application, Interactions, Lookup, Lookups};

    impl<FX, L> From<L> for LeftAssoc<Lookups<FX>, Application<FX>>
    where
        FX: Effect,
        Lookups<FX>: From<L>,
    {
        fn from(left: L) -> Self {
            Self::from((Lookups::from(left), vec![]))
        }
    }

    impl<FX, L> From<L> for LeftAssoc<Interactions<FX>, Lookup>
    where
        FX: Effect,
        Interactions<FX>: From<L>,
    {
        fn from(left: L) -> Self {
            Self::from((Interactions::from(left), vec![]))
        }
    }
}

impl<L, R, T> ParsableWith<T> for LeftAssoc<L, R>
where
    L: ParsableWith<T>,
    R: ParsableWith<T>,
    T: Clone,
{
    fn make_parser_with(t: T) -> impl Parser<Self> {
        L::parser_with(t.clone())
            .then(R::parser_with(t).repeated())
            .map(LeftAssoc::from)
    }
}

impl<L, R> Unparse for LeftAssoc<L, R>
where
    L: Unparse,
    R: Unparse,
{
    fn unparse_into(&self, s: &mut sappho_unparse::Stream) {
        s.write(&self.left);
        for r in &self.rights {
            s.write(r);
        }
    }
}

impl<LS, RS, LT, RT> RestrictFrom<LeftAssoc<LS, RS>> for LeftAssoc<LT, RT>
where
    LT: RestrictFrom<LS>,
    RT: RestrictFrom<RS>,
{
    fn restrict(src: LeftAssoc<LS, RS>) -> Result<LeftAssoc<LT, RT>, Restriction> {
        src.try_map(LT::restrict, RT::restrict)
    }
}
