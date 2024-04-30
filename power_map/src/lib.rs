//! 为有限的定义域提供大小固定的映射
//!
//! [`TotalMap`] 提供了类似 `EnumMap` 的单射。
//!
//! 作为键者需实现 [`MapKey`]，可使用 [`GenericMapKey`] 自动实现。

#![no_std]

mod generic;
mod totalmap;
mod traits;

pub use generic::GenericMapKey;
pub use totalmap::TotalMap;
pub use traits::MapKey;
