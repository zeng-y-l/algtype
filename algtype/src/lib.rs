//! 数据有结构，类型如 struct 或 enum 能表示为类型的积的和。
//! 实现 [`Generic`] 的类型拥有表示 [`Generic::Repr`]， 该类型的数据能与其表示互相转换。
//!
//! [`utils`] 模块提供了有用（其实没啥用）的方法以操作实现 [`Generic`] 的类型。
//!
//! README 有额外信息，[`Generic`] 的文档有详细说明。

#![no_std]

mod generic;
mod impls;
pub mod utils;

pub use algtype_derive::Generic;
pub use generic::*;
