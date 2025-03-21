use crate::leftassoc::LeftAssoc;
use crate::{Applications, BoxWise, Confined, Expr, Interactions, Lookups, Wise};

impl<FX, T> PartialEq<T> for BoxWise<FX>
where
    T: ?Sized,
    Wise<FX>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<FX, T> PartialEq<T> for Wise<FX>
where
    T: ?Sized,
    Expr<FX>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.parsed.eq(other)
    }
}

impl<FX, T> PartialEq<T> for Expr<FX>
where
    T: ?Sized,
    Applications<FX>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        match self {
            Expr::Applications(x) => x.eq(other),
            _ => false,
        }
    }
}

impl<FX, T> PartialEq<T> for Applications<FX>
where
    T: ?Sized,
    Lookups<FX>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<L, R, T> PartialEq<T> for LeftAssoc<L, R>
where
    T: ?Sized,
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

impl<FX, T> PartialEq<T> for Lookups<FX>
where
    T: ?Sized,
    Interactions<FX>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<FX, T> PartialEq<T> for Interactions<FX>
where
    T: ?Sized,
    Confined<FX>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        if self.effects.is_empty() {
            self.confined.eq(other)
        } else {
            false
        }
    }
}

impl<FX> PartialEq<str> for Confined<FX> {
    fn eq(&self, other: &str) -> bool {
        match self {
            Confined::Ref(x) => x.eq(other),
            _ => false,
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
