use crate::generic::{GMapKey, GenericMapKey};
use algtype::{visit_tuple, Generic};
use core::array::from_fn;

/// 可以作为键者
///
/// 建议使用 [`GenericMapKey`] 自动实现。
pub trait MapKey: Clone {
    type MapTy<V>;

    fn k_from_fn<V>(f: impl FnMut(Self) -> V) -> Self::MapTy<V>;

    fn k_as_ref<V>(this: &Self::MapTy<V>) -> Self::MapTy<&V>;
    fn k_as_mut<V>(this: &mut Self::MapTy<V>) -> Self::MapTy<&mut V>;

    fn k_map<V, W>(this: Self::MapTy<V>, f: impl FnMut(V) -> W) -> Self::MapTy<W>;
    fn k_map_with_key<V, W>(this: Self::MapTy<V>, f: impl FnMut(Self, V) -> W) -> Self::MapTy<W>;

    fn k_zip_with<V1, V2, W>(
        a: Self::MapTy<V1>,
        b: Self::MapTy<V2>,
        f: impl FnMut(V1, V2) -> W,
    ) -> Self::MapTy<W>;

    fn k_index<'a, V: 'a>(this: &'a Self::MapTy<V>, k: &Self) -> &'a V;
    fn k_index_mut<'a, V: 'a>(this: &'a mut Self::MapTy<V>, k: &Self) -> &'a mut V;

    type Values<V>: Iterator<Item = V>;

    fn k_into_values<V>(this: Self::MapTy<V>) -> Self::Values<V>;
}

// ADT

impl<T: Clone + MapKey> GenericMapKey for Option<T> {}

impl<T: Clone + MapKey, E: Clone + MapKey> GenericMapKey for Result<T, E> {}

impl GenericMapKey for bool {}

// 数组

impl<T, const N: usize> GenericMapKey for [T; N]
where
    T: Clone,
    Self: Generic,
    Self::Repr: GMapKey,
{
}

// 元组

macro_rules! impl_tuple {
    ($($tys:ident)*) => {
        impl <$($tys: MapKey),*> GenericMapKey for ($($tys,)*) {}
    };
}

visit_tuple!(impl_tuple);

// 数字

macro_rules! impl_number {
    ($($ty:ty)*) => {$(const _: () = {
        const LEN: usize = 1 << <$ty>::BITS;

        impl MapKey for $ty {
            type MapTy<V> = [V; LEN];

            fn k_from_fn<V>(mut f: impl FnMut(Self) -> V) -> Self::MapTy<V> {
                from_fn(|k| f((k as Self).wrapping_add(Self::MIN)))
            }

            fn k_as_ref<V>(this: &Self::MapTy<V>) -> Self::MapTy<&V> {
                // TODO 稳定后使用 `each_ref`
                from_fn(|i| &this[i])
            }

            fn k_as_mut<V>(this: &mut Self::MapTy<V>) -> Self::MapTy<&mut V> {
                // TODO 稳定后使用 `each_mut`
                let mut iter = this.iter_mut();
                from_fn(|_| iter.next().unwrap())
            }

            fn k_map<V, W>(this: Self::MapTy<V>, f: impl FnMut(V) -> W) -> Self::MapTy<W> {
                this.map(f)
            }

            fn k_map_with_key<V, W>(
                this: Self::MapTy<V>,
                mut f: impl FnMut(Self, V) -> W,
            ) -> Self::MapTy<W> {
                let mut k = Self::MIN..=Self::MAX;
                this.map(|v| f(k.next().unwrap(), v))
            }

            fn k_zip_with<V1, V2, W>(
                a: Self::MapTy<V1>,
                b: Self::MapTy<V2>,
                mut f: impl FnMut(V1, V2) -> W,
            ) -> Self::MapTy<W> {
                let mut b = b.into_iter();
                a.map(|a| f(a, b.next().unwrap()))
            }

            #[inline]
            fn k_index<'a, V: 'a>(this: &'a Self::MapTy<V>, k: &Self) -> &'a V {
                &this[k.abs_diff(Self::MIN) as usize]
            }

            #[inline]
            fn k_index_mut<'a, V: 'a>(this: &'a mut Self::MapTy<V>, k: &Self) -> &'a mut V {
                &mut this[k.abs_diff(Self::MIN) as usize]
            }

            type Values<V> = core::array::IntoIter<V, LEN>;

            fn k_into_values<V>(this: Self::MapTy<V>) -> Self::Values<V> {
                this.into_iter()
            }
        }
    };)*};
}

// 更多的就不实现了
impl_number!(i8 u8);
