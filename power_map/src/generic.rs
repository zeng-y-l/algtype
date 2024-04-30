use crate::MapKey;
use algtype::{Generic, One, Product, Repr, Sum, Zero};
use core::iter;

pub trait GMapKey: Sized + Repr {
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

    fn k_index<'a, V: 'a>(this: &'a Self::MapTy<V>, k: &Self::Ref<'_>) -> &'a V;
    fn k_index_mut<'a, V: 'a>(this: &'a mut Self::MapTy<V>, k: &Self::Ref<'_>) -> &'a mut V;

    type Values<V>: Iterator<Item = V>;

    fn k_into_values<V>(this: Self::MapTy<V>) -> Self::Values<V>;
}

impl GMapKey for Zero {
    type MapTy<T> = ();

    fn k_from_fn<V>(_f: impl FnMut(Self) -> V) -> Self::MapTy<V> {}

    #[inline]
    fn k_as_ref<V>(_this: &Self::MapTy<V>) -> Self::MapTy<&V> {}

    #[inline]
    fn k_as_mut<V>(_this: &mut Self::MapTy<V>) -> Self::MapTy<&mut V> {}

    fn k_map<V, W>(_this: Self::MapTy<V>, _f: impl FnMut(V) -> W) -> Self::MapTy<W> {}

    fn k_map_with_key<V, W>(_this: Self::MapTy<V>, _f: impl FnMut(Self, V) -> W) -> Self::MapTy<W> {
    }

    fn k_zip_with<V1, V2, W>(
        _a: Self::MapTy<V1>,
        _b: Self::MapTy<V2>,
        _f: impl FnMut(V1, V2) -> W,
    ) -> Self::MapTy<W> {
    }

    #[inline]
    fn k_index<'a, V: 'a>(_this: &'a Self::MapTy<V>, k: &Self::Ref<'_>) -> &'a V {
        match *k {}
    }

    #[inline]
    fn k_index_mut<'a, V: 'a>(_this: &'a mut Self::MapTy<V>, k: &Self::Ref<'_>) -> &'a mut V {
        match *k {}
    }

    type Values<V> = iter::Empty<V>;

    fn k_into_values<V>(_this: Self::MapTy<V>) -> Self::Values<V> {
        iter::empty()
    }
}

impl GMapKey for One {
    type MapTy<T> = T;

    fn k_from_fn<V>(mut f: impl FnMut(Self) -> V) -> Self::MapTy<V> {
        f(One)
    }

    #[inline]
    fn k_as_ref<V>(this: &Self::MapTy<V>) -> Self::MapTy<&V> {
        this
    }

    #[inline]
    fn k_as_mut<V>(this: &mut Self::MapTy<V>) -> Self::MapTy<&mut V> {
        this
    }

    fn k_map<V, W>(this: Self::MapTy<V>, mut f: impl FnMut(V) -> W) -> Self::MapTy<W> {
        f(this)
    }

    fn k_map_with_key<V, W>(
        this: Self::MapTy<V>,
        mut f: impl FnMut(Self, V) -> W,
    ) -> Self::MapTy<W> {
        f(One, this)
    }

    fn k_zip_with<V1, V2, W>(
        a: Self::MapTy<V1>,
        b: Self::MapTy<V2>,
        mut f: impl FnMut(V1, V2) -> W,
    ) -> Self::MapTy<W> {
        f(a, b)
    }

    #[inline]
    fn k_index<'a, V: 'a>(this: &'a Self::MapTy<V>, _k: &Self::Ref<'_>) -> &'a V {
        this
    }

    #[inline]
    fn k_index_mut<'a, V: 'a>(this: &'a mut Self::MapTy<V>, _k: &Self::Ref<'_>) -> &'a mut V {
        this
    }

    type Values<V> = iter::Once<V>;

    fn k_into_values<V>(this: Self::MapTy<V>) -> Self::Values<V> {
        iter::once(this)
    }
}

impl<T: GMapKey, R: GMapKey> GMapKey for Sum<T, R> {
    type MapTy<V> = (T::MapTy<V>, R::MapTy<V>);

    fn k_from_fn<V>(mut f: impl FnMut(Self) -> V) -> Self::MapTy<V> {
        (
            T::k_from_fn(|k| f(Sum::This(k))),
            R::k_from_fn(|k| f(Sum::Next(k))),
        )
    }

    #[inline]
    fn k_as_ref<V>(this: &Self::MapTy<V>) -> Self::MapTy<&V> {
        (T::k_as_ref(&this.0), R::k_as_ref(&this.1))
    }

    #[inline]
    fn k_as_mut<V>(this: &mut Self::MapTy<V>) -> Self::MapTy<&mut V> {
        (T::k_as_mut(&mut this.0), R::k_as_mut(&mut this.1))
    }

    fn k_map<V, W>(this: Self::MapTy<V>, mut f: impl FnMut(V) -> W) -> Self::MapTy<W> {
        (T::k_map(this.0, &mut f), R::k_map(this.1, &mut f))
    }

    fn k_map_with_key<V, W>(
        this: Self::MapTy<V>,
        mut f: impl FnMut(Self, V) -> W,
    ) -> Self::MapTy<W> {
        (
            T::k_map_with_key(this.0, |k, v| f(Sum::This(k), v)),
            R::k_map_with_key(this.1, |k, v| f(Sum::Next(k), v)),
        )
    }

    fn k_zip_with<V1, V2, W>(
        a: Self::MapTy<V1>,
        b: Self::MapTy<V2>,
        mut f: impl FnMut(V1, V2) -> W,
    ) -> Self::MapTy<W> {
        (
            T::k_zip_with(a.0, b.0, &mut f),
            R::k_zip_with(a.1, b.1, &mut f),
        )
    }

    #[inline]
    fn k_index<'a, V: 'a>(this: &'a Self::MapTy<V>, k: &Self::Ref<'_>) -> &'a V {
        match k {
            Sum::This(k) => T::k_index(&this.0, k),
            Sum::Next(k) => R::k_index(&this.1, k),
        }
    }

    #[inline]
    fn k_index_mut<'a, V: 'a>(this: &'a mut Self::MapTy<V>, k: &Self::Ref<'_>) -> &'a mut V {
        match k {
            Sum::This(k) => T::k_index_mut(&mut this.0, k),
            Sum::Next(k) => R::k_index_mut(&mut this.1, k),
        }
    }

    type Values<V> = iter::Chain<T::Values<V>, R::Values<V>>;

    fn k_into_values<V>(this: Self::MapTy<V>) -> Self::Values<V> {
        T::k_into_values(this.0).chain(R::k_into_values(this.1))
    }
}

impl<T: MapKey, R: GMapKey> GMapKey for Product<T, R> {
    // 如果这里不包装一下就会报生命周期的错。为啥啊？？？
    type MapTy<V> = ProductMap<T, R, V>;

    fn k_from_fn<V>(mut f: impl FnMut(Self) -> V) -> Self::MapTy<V> {
        ProductMap(T::k_from_fn(|l| R::k_from_fn(|r| f(Product(l.clone(), r)))))
    }

    #[inline]
    fn k_as_ref<V>(this: &Self::MapTy<V>) -> Self::MapTy<&V> {
        ProductMap(T::k_map(T::k_as_ref(&this.0), R::k_as_ref))
    }

    #[inline]
    fn k_as_mut<V>(this: &mut Self::MapTy<V>) -> Self::MapTy<&mut V> {
        ProductMap(T::k_map(T::k_as_mut(&mut this.0), R::k_as_mut))
    }

    fn k_map<V, W>(this: Self::MapTy<V>, mut f: impl FnMut(V) -> W) -> Self::MapTy<W> {
        ProductMap(T::k_map(this.0, |this| R::k_map(this, &mut f)))
    }

    fn k_map_with_key<V, W>(
        this: Self::MapTy<V>,
        mut f: impl FnMut(Self, V) -> W,
    ) -> Self::MapTy<W> {
        ProductMap(T::k_map_with_key(this.0, |l, this| {
            R::k_map_with_key(this, |r, v| f(Product(l.clone(), r), v))
        }))
    }

    fn k_zip_with<V1, V2, W>(
        a: Self::MapTy<V1>,
        b: Self::MapTy<V2>,
        mut f: impl FnMut(V1, V2) -> W,
    ) -> Self::MapTy<W> {
        ProductMap(T::k_zip_with(a.0, b.0, |a, b| R::k_zip_with(a, b, &mut f)))
    }

    #[inline]
    fn k_index<'a, V: 'a>(this: &'a Self::MapTy<V>, k: &Self::Ref<'_>) -> &'a V {
        R::k_index(T::k_index(&this.0, k.0), &k.1)
    }

    #[inline]
    fn k_index_mut<'a, V: 'a>(this: &'a mut Self::MapTy<V>, k: &Self::Ref<'_>) -> &'a mut V {
        R::k_index_mut(T::k_index_mut(&mut this.0, k.0), &k.1)
    }

    type Values<V> = iter::Flatten<T::Values<IntoValues<R, V>>>;

    fn k_into_values<V>(this: Self::MapTy<V>) -> Self::Values<V> {
        T::k_into_values(T::k_map(this.0, IntoValues)).flatten()
    }
}

pub struct ProductMap<T: MapKey, R: GMapKey, V>(T::MapTy<R::MapTy<V>>);

/// 实现 [`IntoIterator`] 的包装，用于遍历值
pub struct IntoValues<K: GMapKey, V>(K::MapTy<V>);

impl<K: GMapKey, V> IntoIterator for IntoValues<K, V> {
    type Item = V;
    type IntoIter = K::Values<V>;

    fn into_iter(self) -> Self::IntoIter {
        K::k_into_values(self.0)
    }
}

/// 基于 [`Generic`] 自动实现
///
/// 若某类型实现了 [`Generic`] 和 [`GenericMapKey`]，将会自动实现 [`MapKey`]。
pub trait GenericMapKey: Clone + Generic
where
    Self::Repr: GMapKey,
{
}

impl<T: GenericMapKey> MapKey for T
where
    T::Repr: GMapKey,
{
    type MapTy<V> = <T::Repr as GMapKey>::MapTy<V>;

    fn k_from_fn<V>(mut f: impl FnMut(Self) -> V) -> Self::MapTy<V> {
        T::Repr::k_from_fn(|k| f(T::from_repr(k)))
    }

    #[inline]
    fn k_as_ref<V>(this: &Self::MapTy<V>) -> Self::MapTy<&V> {
        T::Repr::k_as_ref(this)
    }

    #[inline]
    fn k_as_mut<V>(this: &mut Self::MapTy<V>) -> Self::MapTy<&mut V> {
        T::Repr::k_as_mut(this)
    }

    fn k_map<V, W>(this: Self::MapTy<V>, f: impl FnMut(V) -> W) -> Self::MapTy<W> {
        T::Repr::k_map(this, f)
    }

    fn k_map_with_key<V, W>(
        this: Self::MapTy<V>,
        mut f: impl FnMut(Self, V) -> W,
    ) -> Self::MapTy<W> {
        T::Repr::k_map_with_key(this, |k, v| f(T::from_repr(k), v))
    }

    fn k_zip_with<V1, V2, W>(
        a: Self::MapTy<V1>,
        b: Self::MapTy<V2>,
        f: impl FnMut(V1, V2) -> W,
    ) -> Self::MapTy<W> {
        T::Repr::k_zip_with(a, b, f)
    }

    #[inline]
    fn k_index<'a, V: 'a>(this: &'a Self::MapTy<V>, k: &Self) -> &'a V {
        T::Repr::k_index(this, &k.as_repr())
    }

    #[inline]
    fn k_index_mut<'a, V: 'a>(this: &'a mut Self::MapTy<V>, k: &Self) -> &'a mut V {
        T::Repr::k_index_mut(this, &k.as_repr())
    }

    type Values<V> = <T::Repr as GMapKey>::Values<V>;

    fn k_into_values<V>(this: Self::MapTy<V>) -> Self::Values<V> {
        T::Repr::k_into_values(this)
    }
}
