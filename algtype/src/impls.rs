use crate::{Generic, One, Product, Repr, Sum, Zero};
use algtype_derive::impl_generic;

impl Generic for bool {
    type Repr = Sum<One, Sum<One, Zero>>;

    #[inline]
    fn into_repr(self) -> Self::Repr {
        if self {
            Sum::Next(Sum::This(One))
        } else {
            Sum::This(One)
        }
    }

    #[inline]
    fn from_repr(repr: Self::Repr) -> Self {
        match repr {
            Sum::This(One) => false,
            Sum::Next(Sum::This(One)) => true,
            Sum::Next(Sum::Next(a)) => match a {},
        }
    }

    #[inline]
    fn as_repr(&self) -> <Self::Repr as Repr>::Ref<'_> {
        self.into_repr()
    }

    #[inline]
    fn as_mut_repr(&mut self) -> <Self::Repr as Repr>::Mut<'_> {
        self.into_repr()
    }
}

// ADT

impl_generic!(
    enum Option<T> {
        None,
        Some(T),
    }
);

impl_generic!(
    enum Result<T, E> {
        Err(E),
        Ok(T),
    }
);

// 元组

/// 遍历不同长度的元组
///
/// 用于在元组上实现各种功能。
#[macro_export]
macro_rules! visit_tuple {
    ($cb:ident) => {
        visit_tuple!(@many $cb, T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
    };
    (@many $cb:ident $($tys:ident)*, $ty:ident $($tys2:ident)*) => {
        $cb!($($tys)*);
        visit_tuple!(@many $cb $($tys)* $ty, $($tys2)*);
    };
    (@many $cb:ident $($tys:ident)*,) => {
        $cb!($($tys)*);
    };
}

macro_rules! impl_tuple {
    (@ty) => {
        One
    };
    (@ty $ty:ident $($tys:ident)*) => {
        Product<$ty, impl_tuple!(@ty $($tys)*)>
    };
    (@val) => {
        One
    };
    (@val $val:ident $($vals:ident)*) => {
        Product($val, impl_tuple!(@val $($vals)*))
    };
    ($($tys:ident)*) => {
        #[allow(non_snake_case)]
        impl<$($tys,)*> Generic for ($($tys,)*) {
            type Repr = Sum<impl_tuple!(@ty $($tys)*), Zero>;

            #[inline]
            fn into_repr(self) -> Self::Repr {
                let ($($tys,)*) = self;
                Sum::This(impl_tuple!(@val $($tys)*))
            }

            #[inline]
            fn from_repr(repr: Self::Repr) -> Self {
                match repr {
                    Sum::This(impl_tuple!(@val $($tys)*)) => ($($tys,)*),
                    Sum::Next(a) => match a {},
                }
            }

            #[inline]
            fn as_repr(&self) -> <Self::Repr as Repr>::Ref<'_> {
                let ($($tys,)*) = self;
                Sum::This(impl_tuple!(@val $($tys)*))
            }

            #[inline]
            fn as_mut_repr(&mut self) -> <Self::Repr as Repr>::Mut<'_> {
                let ($($tys,)*) = self;
                Sum::This(impl_tuple!(@val $($tys)*))
            }
        }
    };
}

visit_tuple!(impl_tuple);

// 数组

macro_rules! impl_array {
    (@many $($szs:literal $nms:ident)*, $sz:literal $nm:ident $sz2:literal $($nms2:ident $szs2:literal)*) => {
        impl_array!($sz $($nms)*);
        impl_array!(@many $($szs $nms)* $sz $nm, $sz2 $($nms2 $szs2)*);
    };
    (@many $($szs:literal $nms:ident)*, $sz:literal) => {
        impl_array!($sz $($nms)*);
    };
    (@ty) => {
        One
    };
    (@ty $nm:ident $($nms:ident)*) => {
        Product<T, impl_array!(@ty $($nms)*)>
    };
    (@val) => {
        One
    };
    (@val $nm:ident $($nms:ident)*) => {
        Product($nm, impl_array!(@val $($nms)*))
    };
    ($sz:literal $($nms:ident)*) => {
        impl<T> Generic for [T; $sz] {
            type Repr = Sum<impl_array!(@ty $($nms)*), Zero>;

            #[inline]
            fn into_repr(self) -> Self::Repr {
                let [$($nms,)*] = self;
                Sum::This(impl_array!(@val $($nms)*))
            }

            #[inline]
            fn from_repr(repr: Self::Repr) -> Self {
                match repr {
                    Sum::This(impl_array!(@val $($nms)*)) => [$($nms,)*],
                    Sum::Next(a) => match a {},
                }
            }

            #[inline]
            fn as_repr(&self) -> <Self::Repr as Repr>::Ref<'_> {
                let [$($nms,)*] = self;
                Sum::This(impl_array!(@val $($nms)*))
            }

            #[inline]
            fn as_mut_repr(&mut self) -> <Self::Repr as Repr>::Mut<'_> {
                let [$($nms,)*] = self;
                Sum::This(impl_array!(@val $($nms)*))
            }
        }
    };
}

impl_array!(@many, 0 x0 1 x1 2 x2 3 x3 4 x4 5 x5 6 x6 7 x7 8 x8 9 x9 10 x10 11 x11 12);
