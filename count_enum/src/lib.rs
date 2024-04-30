//! 操作值域有限的类型
//!
//! 它给值域一个顺序，序号和迭代之类都按照这个顺序。
//!
//! [`Enum`] 提供了基础的方法，可使用 [`GenericEnum`] 自动实现。
//! 使用 [`iter_each`] 或 [`iter_each_from`] 迭代 [`Enum`] 的值域。

#![no_std]

mod generic;
mod impls;
mod iter;

use core::num::NonZeroUsize;
pub use generic::GenericEnum;
pub use iter::{iter_each, iter_each_from, IterEachFrom};

/// 枚举
///
/// 建议使用 [`GenericEnum`] 自动实现。
pub trait Enum: Clone {
    /// 类型的取值的总数
    ///
    /// 如果溢出，则为 `None`。
    ///
    /// ```
    /// # use count_enum::Enum;
    /// assert_eq!(Option::<bool>::CARD, Some(3));
    /// assert_eq!(i128::CARD, None);
    /// ```
    const CARD: Option<usize>;

    /// 值的序号
    ///
    /// 返回值应小于 `CARD`（如果有）。如果溢出，则返回 `None`。
    ///
    /// ```
    /// # use count_enum::Enum;
    /// assert_eq!(Some(true).to_index(), Some(2));
    /// assert_eq!(12i8.to_index(), Some(140));
    /// assert_eq!(0u128.to_index(), Some(0));
    /// assert_eq!(u128::MAX.to_index(), None);
    /// ```
    fn to_index(&self) -> Option<usize>;

    /// 序号对应的值
    ///
    /// 输入应小于 `CARD`（如果有），否则返回 `None`。
    ///
    /// ```
    /// # use count_enum::Enum;
    /// assert_eq!(Option::<bool>::from_index(2), Some(Some(true)));
    /// assert_eq!(i8::from_index(0), Some(-128));
    /// assert_eq!(i16::from_index(99999), None);
    /// ```
    fn from_index(i: usize) -> Option<Self>;

    /// 第一个值
    ///
    /// 如果值域为空，则返回 `None`。
    ///
    /// ```
    /// # use count_enum::Enum;
    /// assert_eq!(i8::first(), Some(-128));
    /// ```
    fn first() -> Option<Self>;

    /// 最后一个值
    ///
    /// 如果值域为空，则返回 `None`。
    ///
    /// ```
    /// # use count_enum::Enum;
    /// assert_eq!(i8::last(), Some(127));
    /// ```
    fn last() -> Option<Self>;

    /// 上一个值
    ///
    /// 如果已经是第一个值，则返回 `None`。
    ///
    /// ```
    /// # use count_enum::Enum;
    /// assert_eq!(0i8.prev(), Some(-1));
    /// ```
    fn prev(&self) -> Option<Self>;

    /// 下一个值
    ///
    /// 如果已经是最后一个值，则返回 `None`。
    ///
    /// ```
    /// # use count_enum::Enum;
    /// assert_eq!(0i8.succ(), Some(1));
    /// ```
    fn succ(&self) -> Option<Self>;

    /// 此后的值的总数
    ///
    /// 总数包含此值，因此非零。如果溢出，则返回 `None`。
    ///
    /// ```
    /// # use count_enum::Enum;
    /// assert_eq!(bool::count_from(&false), Some(2.try_into().unwrap()));
    /// ```
    fn count_from(from: &Self) -> Option<NonZeroUsize>;

    /// 遍历每一个值
    ///
    /// 相当于 `iter_each().fold(init, f)`。
    fn fold_each<B, F>(init: B, f: F) -> B
    where
        F: FnMut(B, Self) -> B;

    /// 从某个值开始遍历每一个值
    ///
    /// 相当于 `iter_each_from(from).fold(init, f)`。
    fn fold_each_from<B, F>(from: &Self, init: B, f: F) -> B
    where
        F: FnMut(B, Self) -> B;
}
