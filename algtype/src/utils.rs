//! [`Generic`] 上可用的工具

use crate::{Generic, One, Product, Sum, Zero};

/// 构造 newtype
///
/// ```
/// # use algtype::utils::singleton;
/// let a: [_; 1] = singleton(114);
/// assert_eq!(a, [114]);
/// let a: (_,) = singleton(514);
/// assert_eq!(a, (514,));
/// ```
pub fn singleton<T, U: Generic<Repr = Sum<Product<T, One>, Zero>>>(x: T) -> U {
    U::from_repr(Sum::This(Product(x, One)))
}

/// 转换表示一样的类型
///
/// ```
/// # use algtype::utils::cast;
/// let a: [_; 2] = cast((1919, 810));
/// assert_eq!(a, [1919, 810]);
/// ```
pub fn cast<T: Generic, U: Generic<Repr = T::Repr>>(x: T) -> U {
    U::from_repr(x.into_repr())
}

trait VariantCount {
    const COUNT: u32;
}

impl VariantCount for Zero {
    const COUNT: u32 = 0;
}

impl<T, R: VariantCount> VariantCount for Sum<T, R> {
    const COUNT: u32 = 1 + R::COUNT;
}

/// 获取变体的数量
///
/// ```
/// # use algtype::utils::variant_count;
/// assert_eq!(variant_count::<Option<()>>(), 2);
/// assert_eq!(variant_count::<()>(), 1);
/// ```
#[allow(private_bounds)]
pub const fn variant_count<T>() -> u32
where
    T: Generic,
    T::Repr: VariantCount,
{
    T::Repr::COUNT
}
