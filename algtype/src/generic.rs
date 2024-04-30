/// 可作为表示的类型
///
/// 它是 [`Sum`]、[`Zero`]、[`Product`]、[`One`] 之一。
///
/// 不可能从 `&T` 转换成 `&T::Repr`，
/// 因此提供了 `Repr::Ref` 为表示的引用形式，`Repr::Mut` 也类似。
///
/// 之所以不使用现成的类型（如 `(T, R)` 和 `Either<T, R>`），是为了方便。
/// 如果使用 `(T, R)`，就无法通过宏来一次性在各种长度的元组上实现某 trait。
pub trait Repr {
    /// 给底层类型加上引用
    type Ref<'a>: Repr
    where
        Self: 'a;
    /// 给底层类型加上可变引用
    type Mut<'a>: Repr
    where
        Self: 'a;

    /// 给底层类型加上引用
    fn as_ref(&self) -> Self::Ref<'_>;
    /// 给底层类型加上可变引用
    fn as_mut_ref(&mut self) -> Self::Mut<'_>;
}

/// 空类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Zero {}

/// 和类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sum<T, R> {
    This(T),
    Next(R),
}

/// 单元类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct One;

/// 积类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Product<T, R>(pub T, pub R);

impl Repr for Zero {
    type Ref<'a> = Zero;
    type Mut<'a> = Zero;

    fn as_ref(&self) -> Self::Ref<'_> {
        match *self {}
    }
    fn as_mut_ref(&mut self) -> Self::Mut<'_> {
        match *self {}
    }
}

impl Repr for One {
    type Ref<'a> = One;
    type Mut<'a> = One;

    fn as_ref(&self) -> Self::Ref<'_> {
        One
    }
    fn as_mut_ref(&mut self) -> Self::Mut<'_> {
        One
    }
}

impl<T: Repr, R: Repr> Repr for Sum<T, R> {
    type Ref<'a> = Sum<T::Ref<'a>, R::Ref<'a>>
    where T: 'a, R: 'a;
    type Mut<'a> = Sum<T::Mut<'a>, R::Mut<'a>>
    where T: 'a, R: 'a;

    fn as_ref(&self) -> Self::Ref<'_> {
        match self {
            Sum::This(x) => Sum::This(x.as_ref()),
            Sum::Next(x) => Sum::Next(x.as_ref()),
        }
    }
    fn as_mut_ref(&mut self) -> Self::Mut<'_> {
        match self {
            Sum::This(x) => Sum::This(x.as_mut_ref()),
            Sum::Next(x) => Sum::Next(x.as_mut_ref()),
        }
    }
}

impl<T, R: Repr> Repr for Product<T, R> {
    type Ref<'a> = Product<&'a T, R::Ref<'a>>
    where T: 'a, R: 'a;
    type Mut<'a>  = Product<&'a mut T, R::Mut<'a>>
    where T: 'a, R: 'a;

    fn as_ref(&self) -> Self::Ref<'_> {
        Product(&self.0, self.1.as_ref())
    }
    fn as_mut_ref(&mut self) -> Self::Mut<'_> {
        Product(&mut self.0, self.1.as_mut_ref())
    }
}

/// 类型与其表示的互转
///
/// [`Generic::Repr`] 提供了类型的*表示*，即其数据的结构。
///
/// 拥有某类型的数据，可以通过 [`Generic::into_repr`] 转换成其表示，
/// 或通过 [`Generic::from_repr`] 转换回来。
/// 若有数据的引用，可以通过 [`Generic::as_repr`] 转换成引用形式的表示；
/// [`Generic::as_mut_repr`] 也类似。注意，引用形式的表示无法转换回来。
///
/// 可以使用 derive 宏，在 struct 或 enum 上自动实现之。
/// 它无法实现于 `&T` 或 `&mut T`，因为无法实现 `from_repr`。
///
/// 类型的表示，是其底层类型的积的和，呈现为类型层面的列表。
/// 它只体现结构，不包含其他信息，例如字段或变体的名字、枚举的值之类。
/// 表示的伪代码如下：
///
/// ```txt
/// repr ::= sums
/// sums ::= Zero | Sum<products, sums>
/// products ::= One | Product<type, products>
/// type ::= 任何类型，如 i32、()、Vec<String>
/// ```
///
/// # 举例
///
/// ```
/// # use algtype::{Generic, Sum, Product, One};
/// #[derive(Generic, Debug, PartialEq)]
/// enum E {
///     A(i32),
///     B(bool),
///     C,
/// }
///
/// assert_eq!(None::<i32>.into_repr(), Sum::This(One));
/// assert_eq!(<[_; 2]>::from_repr(Sum::This(Product(true, Product(false, One)))), [true, false]);
/// assert_eq!(E::B(true).as_repr(), Sum::Next(Sum::This(Product(&true, One))));
/// assert_eq!((1, E::A(1)).as_mut_repr(), Sum::This(Product(&mut 1, Product(&mut E::A(1), One))));
/// ```
pub trait Generic {
    /// 类型的表示
    type Repr: Repr;
    /// 把数据转换成其表示
    fn into_repr(self) -> Self::Repr;
    /// 从数据的表示转换成数据
    fn from_repr(repr: Self::Repr) -> Self;
    /// 获取数据的表示的引用形式
    fn as_repr(&self) -> <Self::Repr as Repr>::Ref<'_>;
    /// 获取数据的表示的可变引用形式
    fn as_mut_repr(&mut self) -> <Self::Repr as Repr>::Mut<'_>;
}
