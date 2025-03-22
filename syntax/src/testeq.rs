use either::Either::{Left, Right};

use crate::leftassoc::LeftAssoc;
use crate::{Applications, BoxWise, Confined, Expr, Interactions, Lookups, Wise};

impl<FX, T> PartialEq<T> for BoxWise<FX>
where
    T: ?Sized,
    Confined<FX>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<FX, T> PartialEq<T> for Wise<FX>
where
    T: ?Sized,
    Confined<FX>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.parsed.eq(other)
    }
}

impl<FX, T> PartialEq<T> for Expr<FX>
where
    T: ?Sized,
    Confined<FX>: PartialEq<T>,
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
    Confined<FX>: PartialEq<T>,
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
    Confined<FX>: PartialEq<T>,
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

// `Confined` comparables
//
// These "terminate" the expression type hierarchy, although they can be recursive, since confined expressions can contain general parenthesized expressions.
impl<FX, T, const K: usize> PartialEq<[T; K]> for Confined<FX>
where
    Confined<FX>: PartialEq<T>,
{
    fn eq(&self, other: &[T; K]) -> bool {
        <Self as PartialEq<[T]>>::eq(self, other.as_slice())
    }
}

impl<FX, T> PartialEq<[T]> for Confined<FX>
where
    Confined<FX>: PartialEq<T>,
{
    fn eq(&self, other: &[T]) -> bool {
        match self {
            Confined::ListExpr(x) => {
                let mut it = other.iter();

                for subx in x.iter() {
                    if let Some(other) = it.next() {
                        let eq = match subx {
                            Left(l) => l.eq(other),
                            Right(r) => r.eq(other),
                        };
                        if !eq {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // `x.iter()` is complete, so we match if `it` is complete:
                it.next().is_none()
            }
            _ => false,
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
