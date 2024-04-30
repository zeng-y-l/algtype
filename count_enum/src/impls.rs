use crate::{generic::GEnum, Enum, GenericEnum};
use algtype::{visit_tuple, Generic};
use core::num::NonZeroUsize;

// ADT

impl<T: Clone + Enum> GenericEnum for Option<T> {}

impl<T: Clone + Enum, E: Clone + Enum> GenericEnum for Result<T, E> {}

impl GenericEnum for bool {}

// 数组

impl<T, const N: usize> GenericEnum for [T; N]
where
    T: Clone,
    Self: Generic,
    Self::Repr: GEnum,
{
}

// 元组

macro_rules! impl_tuple {
    ($($tys:ident)*) => {
        impl <$($tys: Enum),*> GenericEnum for ($($tys,)*) {}
    };
}

visit_tuple!(impl_tuple);

// 各种数字

macro_rules! impl_number {
    ($($ty:ty)*) => {$(
        impl Enum for $ty {
            const CARD: Option<usize> = 1usize.checked_shl(Self::BITS);

            #[inline]
            fn to_index(&self) -> Option<usize> {
                self.abs_diff(Self::MIN).try_into().ok()
            }

            #[inline]
            fn from_index(i: usize) -> Option<Self> {
                (Self::MIN..=Self::MAX).nth(i)
            }

            #[inline]
            fn first() -> Option<Self> {
                Some(Self::MIN)
            }

            #[inline]
            fn last() -> Option<Self> {
                Some(Self::MAX)
            }

            #[inline]
            fn prev(&self) -> Option<Self> {
                self.checked_sub(1)
            }

            #[inline]
            fn succ(&self) -> Option<Self> {
                self.checked_add(1)
            }

            #[inline]
            fn count_from(from: &Self) -> Option<NonZeroUsize> {
                let c = Self::MAX.abs_diff(*from).try_into().ok()?;
                NonZeroUsize::new(1).unwrap().checked_add(c)
            }

            fn fold_each<B, F>(init: B, f: F) -> B
            where
                F: FnMut(B, Self) -> B
            {
                (Self::MIN..=Self::MAX).fold(init, f)
            }

            fn fold_each_from<B, F>(from: &Self, init: B, f: F) -> B
            where
                F: FnMut(B, Self) -> B
            {
                (*from..=Self::MAX).fold(init, f)
            }
        }
    )*};
}

impl_number!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
