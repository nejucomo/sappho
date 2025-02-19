use array_init::try_array_init;
use sappho_identifier::IdentRef;

use crate::error::AttrsResult;
use crate::Attrs;

pub trait AttrsKey<T>: Sized {
    type Output;

    fn take_from(self, attrs: &mut Attrs<T>) -> AttrsResult<Self::Output>;
}

impl<T> AttrsKey<T> for &IdentRef {
    type Output = T;

    fn take_from(self, attrs: &mut Attrs<T>) -> AttrsResult<T> {
        attrs.take_basic(self)
    }
}

impl<T> AttrsKey<T> for &'static str {
    type Output = T;

    fn take_from(self, attrs: &mut Attrs<T>) -> AttrsResult<Self::Output> {
        attrs.take(IdentRef::from_static(self))
    }
}

/// Allow _fully empty_ attrs to result in a `None` output, otherwise take the full key
///
/// # Panics
///
/// The key must be `Some`, or else this impl panics
impl<T, K> AttrsKey<T> for Option<K>
where
    K: AttrsKey<T>,
{
    type Output = Option<K::Output>;

    fn take_from(self, attrs: &mut Attrs<T>) -> AttrsResult<Self::Output> {
        let k = self.unwrap();
        if attrs.is_empty() {
            Ok(None)
        } else {
            attrs.take(k).map(Some)
        }
    }
}

impl<T, K, const S: usize> AttrsKey<T> for [K; S]
where
    K: AttrsKey<T>,
{
    type Output = [K::Output; S];

    fn take_from(self, attrs: &mut Attrs<T>) -> AttrsResult<Self::Output> {
        let mut itself = self.into_iter().enumerate();
        let res = try_array_init(|ix| {
            let (expix, k) = itself.next().unwrap();
            assert_eq!(ix, expix);

            k.take_from(attrs)
        });
        assert!(itself.next().is_none());
        res
    }
}

impl<T, A, B> AttrsKey<T> for (A, B)
where
    A: AttrsKey<T>,
    B: AttrsKey<T>,
{
    type Output = (A::Output, B::Output);

    fn take_from(self, attrs: &mut Attrs<T>) -> AttrsResult<Self::Output> {
        let (ak, bk) = self;
        let aval = attrs.take(ak)?;
        let bval = attrs.take(bk)?;
        Ok((aval, bval))
    }
}
