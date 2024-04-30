use crate::MapKey;
use core::{
    cmp::Ordering,
    fmt, hash, iter,
    ops::{Index, IndexMut},
};
use count_enum::{iter_each, Enum, IterEachFrom};

/// 固定大小的全映射表
///
/// 需要注意：
///
/// - 占空间大小与键的值域大小成正比，小心栈的使用。
/// - 目前，创建引用迭代器会调用 `as_ref`、`as_mut`，或将大量使用栈。
/// - 由于使用了 [`Iterator::chain`]，因而迭代可能生成较繁琐的代码。
///   迭代键值对以及使用 `for` 循环都会如此。
///
/// 总之，很不建议使用值域较大的键，例如 `i8`。
pub struct TotalMap<K: MapKey, V>(K::MapTy<V>);

impl<K: MapKey, V> TotalMap<K, V> {
    /// 复制生成每个值
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<bool, i32>::new(42);
    /// assert_eq!(m[&true], 42);
    /// ```
    pub fn new(v: V) -> Self
    where
        V: Clone,
    {
        Self::from_fn(|_| v.clone())
    }

    /// 用函数生成每个值
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<Option<bool>, bool>::from_fn(|k| k.is_some());
    /// assert_eq!(m[&Some(false)], true);
    /// assert_eq!(m[&None], false);
    /// ```
    pub fn from_fn(f: impl FnMut(K) -> V) -> Self {
        Self(K::k_from_fn(f))
    }

    /// 给每个值加上引用
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<(), _>::new(42);
    /// let m = m.as_ref();
    /// assert_eq!(m[&()], &42);
    /// ```
    pub fn as_ref(&self) -> TotalMap<K, &V> {
        TotalMap(K::k_as_ref(&self.0))
    }

    /// 给每个值加上可变引用
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let mut m = TotalMap::<(), _>::new(42);
    /// let m = m.as_mut();
    /// assert_eq!(m[&()], &mut 42);
    /// ```
    pub fn as_mut(&mut self) -> TotalMap<K, &mut V> {
        TotalMap(K::k_as_mut(&mut self.0))
    }

    /// 执行映射
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<(), _>::new(-1);
    /// let m = m.map(i32::is_positive);
    /// assert_eq!(m, TotalMap::new(false));
    /// ```
    pub fn map<W>(self, f: impl FnMut(V) -> W) -> TotalMap<K, W> {
        TotalMap(K::k_map(self.0, f))
    }

    /// 执行映射而包括键
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<bool, _>::new(10);
    /// let m = m.map_with_key(|k, v| v * 2 + (k as i32));
    /// assert_eq!(m[&true], 21);
    /// ```
    pub fn map_with_key<W>(self, f: impl FnMut(K, V) -> W) -> TotalMap<K, W> {
        TotalMap(K::k_map_with_key(self.0, f))
    }

    /// 以引用执行映射
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<i8, String>::from_fn(|x| x.to_string());
    /// let m = m.map_ref(|v| v.contains('0'));
    /// assert_eq!(m[&10], true);
    /// assert_eq!(m[&42], false);
    /// ```
    pub fn map_ref<W>(&self, f: impl FnMut(&V) -> W) -> TotalMap<K, W> {
        TotalMap(K::k_map(self.as_ref().0, f))
    }

    /// 压缩两个表
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::new(1);
    /// let n = TotalMap::new(2);
    /// let o = m.zip(n);
    /// assert_eq!(o[&()], (1, 2));
    /// ```
    pub fn zip<W>(self, that: TotalMap<K, W>) -> TotalMap<K, (V, W)> {
        self.zip_with(that, |a, b| (a, b))
    }

    /// 以函数压缩两个表
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::new(1);
    /// let n = TotalMap::new(2);
    /// let o = m.zip_with(n, |x, y| x + y);
    /// assert_eq!(o[&()], 3);
    /// ```
    pub fn zip_with<V2, W>(
        self,
        that: TotalMap<K, V2>,
        f: impl FnMut(V, V2) -> W,
    ) -> TotalMap<K, W> {
        TotalMap(K::k_zip_with(self.0, that.0, f))
    }

    /// 值的迭代器
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<Option<bool>, _>::from_fn(|x| x.is_some());
    /// assert!(m.into_values().eq([false, true, true]));
    /// ```
    pub fn into_values(self) -> K::Values<V> {
        K::k_into_values(self.0)
    }

    /// 值的引用迭代器
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<Option<bool>, _>::from_fn(|x| x.is_some());
    /// assert!(m.values().eq(&[false, true, true]));
    /// ```
    pub fn values(&self) -> K::Values<&V> {
        K::k_into_values(self.as_ref().0)
    }

    /// 值的可变引用迭代器
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let mut m = TotalMap::<Option<bool>, _>::from_fn(|x| x.is_some());
    /// assert!(m.values_mut().eq(&mut [false, true, true]));
    /// ```
    pub fn values_mut(&mut self) -> K::Values<&mut V> {
        K::k_into_values(self.as_mut().0)
    }
}

impl<K: MapKey + Enum, V> TotalMap<K, V> {
    /// 映射的项目数
    ///
    /// 即键的值域大小，如果溢出则为空。
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// assert_eq!(TotalMap::<Option<bool>, ()>::LEN, Some(3));
    /// ```
    pub const LEN: Option<usize> = K::CARD;

    /// 键和值的引用迭代器
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let m = TotalMap::<Option<bool>, _>::from_fn(|x| x.is_some());
    /// assert!(m.iter().eq([(None, &false), (Some(false), &true), (Some(true), &true)]));
    /// ```
    pub fn iter(&self) -> iter::Zip<IterEachFrom<K>, K::Values<&V>> {
        iter_each().zip(self.values())
    }

    /// 键和值的可变引用迭代器
    ///
    /// ```
    /// # use power_map::TotalMap;
    /// let mut m = TotalMap::<Option<bool>, _>::from_fn(|x| x.is_some());
    /// assert!(m.iter_mut().eq([(None, &mut false), (Some(false), &mut true), (Some(true), &mut true)]));
    /// ```
    pub fn iter_mut(&mut self) -> iter::Zip<IterEachFrom<K>, K::Values<&mut V>> {
        iter_each().zip(self.values_mut())
    }
}

impl<K: MapKey, V> Index<&K> for TotalMap<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        K::k_index(&self.0, index)
    }
}

impl<K: MapKey, V> IndexMut<&K> for TotalMap<K, V> {
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        K::k_index_mut(&mut self.0, index)
    }
}

impl<K: MapKey, V: Clone> Clone for TotalMap<K, V> {
    fn clone(&self) -> Self {
        self.map_ref(V::clone)
    }
}

impl<K: MapKey, V: Default> Default for TotalMap<K, V> {
    fn default() -> Self {
        Self::from_fn(|_| V::default())
    }
}

impl<K: MapKey, V: PartialEq> PartialEq for TotalMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.values().eq(other.values())
    }
}

impl<K: MapKey, V: Eq> Eq for TotalMap<K, V> {}

impl<K: MapKey, V: PartialOrd> PartialOrd for TotalMap<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.values().partial_cmp(other.values())
    }
}

impl<K: MapKey, V: Ord> Ord for TotalMap<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.values().cmp(other.values())
    }
}

impl<K: MapKey, V: hash::Hash> hash::Hash for TotalMap<K, V> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.values().for_each(|v| v.hash(state));
    }
}

impl<K: MapKey + Enum + fmt::Debug, V: fmt::Debug> fmt::Debug for TotalMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'a, K: MapKey, V> Extend<(&'a K, V)> for TotalMap<K, V> {
    fn extend<T: IntoIterator<Item = (&'a K, V)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(k, v)| self[k] = v);
    }
}

impl<'a, K: MapKey, V: Default> FromIterator<(&'a K, V)> for TotalMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (&'a K, V)>>(iter: T) -> Self {
        let mut this = Self::default();
        this.extend(iter);
        this
    }
}

impl<K: MapKey + Enum, V> IntoIterator for TotalMap<K, V> {
    type Item = (K, V);
    type IntoIter = iter::Zip<IterEachFrom<K>, K::Values<V>>;

    fn into_iter(self) -> Self::IntoIter {
        iter_each().zip(self.into_values())
    }
}

impl<'a, K: MapKey + Enum, V> IntoIterator for &'a TotalMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = iter::Zip<IterEachFrom<K>, K::Values<&'a V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: MapKey + Enum, V> IntoIterator for &'a mut TotalMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = iter::Zip<IterEachFrom<K>, K::Values<&'a mut V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
