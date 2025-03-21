use crate::leftassoc::LeftAssoc;
use crate::{Applications, BoxWise, Confined, Expr, Interactions, Lookups, Wise};

impl<FX> PartialEq<i32> for BoxWise<FX> {
    fn eq(&self, other: &i32) -> bool {
        self.0.eq(other)
    }
}

impl<FX> PartialEq<i32> for Wise<FX> {
    fn eq(&self, other: &i32) -> bool {
        self.0.parsed.eq(other)
    }
}

impl<FX> PartialEq<i32> for Expr<FX> {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Expr::Applications(x) => x.eq(other),
            _ => false,
        }
    }
}

impl<FX> PartialEq<i32> for Applications<FX> {
    fn eq(&self, other: &i32) -> bool {
        self.0.eq(other)
    }
}

impl<L, R, T> PartialEq<T> for LeftAssoc<L, R>
where
    L: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        if self.is_just_left() {
            self.ref_left().eq(other)
        } else {
            false
        }
    }
}

impl<FX> PartialEq<i32> for Lookups<FX> {
    fn eq(&self, other: &i32) -> bool {
        self.0.eq(other)
    }
}

impl<FX> PartialEq<i32> for Interactions<FX> {
    fn eq(&self, other: &i32) -> bool {
        if self.effects.is_empty() {
            self.confined.eq(other)
        } else {
            false
        }
    }
}

impl<FX> PartialEq<i32> for Confined<FX> {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Confined::Prim(x) => x.eq(other),
            _ => false,
        }
    }
}
