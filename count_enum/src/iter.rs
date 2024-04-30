use crate::Enum;
use core::iter::FusedIterator;

/// 遍历值域，参见 [`IterEachFrom`]
pub fn iter_each<T: Enum>() -> IterEachFrom<T> {
    IterEachFrom(T::first())
}

/// 从某个值（包含）开始遍历值域，参见 [`IterEachFrom`]
pub fn iter_each_from<T: Enum>(x: T) -> IterEachFrom<T> {
    IterEachFrom(Some(x))
}

/// 遍历值域
///
/// 这是单向迭代器。
/// 实现了 [`ExactSizeIterator`]，但其实际长度有可能溢出。
///
/// 其内部迭代（如 `fold`、`for_each`）经过优化，性能更高，可代替 `for` 循环。
///
/// ```
/// # use count_enum::{iter_each, iter_each_from};
/// assert!(iter_each::<Result<_, _>>()
///     .eq([Err(()), Ok(None), Ok(Some(()))]));
/// assert!(iter_each_from((false, true))
///     .eq([(false, true), (true, false), (true, true)]));
/// ```
///
/// ```should_panic
/// # use count_enum::iter_each;
/// iter_each::<u128>().count(); // overflow
/// ```
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct IterEachFrom<T>(Option<T>);

impl<T: Enum> Iterator for IterEachFrom<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let this = self.0.take()?;
        self.0 = this.succ();
        Some(this)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let Some(this) = &self.0 else {
            return (0, Some(0));
        };
        match T::count_from(this) {
            Some(c) => (c.get(), Some(c.get())),
            None => (usize::MAX, None),
        }
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.map_or(0, |this| {
            T::count_from(&this).expect("count overflow").get()
        })
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.0?;
        T::last()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        match self.0 {
            Some(this) => T::fold_each_from(&this, init, f),
            None => init,
        }
    }
}

impl<T: Enum> FusedIterator for IterEachFrom<T> {}
