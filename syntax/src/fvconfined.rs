macro_rules! impl_from_via_confined {
    ( $t:ty ) => {
        impl<FX> From<i32> for $t
        where
            FX: Effect,
        {
            fn from(v: i32) -> Self {
                <$t>::from(crate::Confined::<FX>::from(v))
            }
        }

        impl<FX> From<&'static str> for $t
        where
            FX: Effect,
        {
            fn from(v: &'static str) -> Self {
                <$t>::from(crate::Confined::<FX>::from(v))
            }
        }

        impl<FX> From<()> for $t
        where
            FX: Effect,
        {
            fn from((): ()) -> Self {
                <$t>::from(crate::Confined::<FX>::from(()))
            }
        }

        impl<FX, T, const K: usize> From<[T; K]> for $t
        where
            FX: Effect,
            crate::Confined<FX>: From<[T; K]>,
        {
            fn from(v: [T; K]) -> Self {
                <$t>::from(crate::Confined::<FX>::from(v))
            }
        }

        impl<FX, A, B> From<(A, B)> for $t
        where
            FX: Effect,
            crate::Wise<FX>: From<A> + From<B>,
        {
            fn from(v: (A, B)) -> Self {
                <$t>::from(crate::Confined::<FX>::from(v))
            }
        }
    };
}
