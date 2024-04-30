use crate::Enum;
use algtype::{Generic, One, Product, Repr, Sum, Zero};
use core::num::NonZeroUsize;

pub trait GEnum: Repr + Sized {
    const CARD: Option<usize>;
    fn to_index(this: &Self::Ref<'_>) -> Option<usize>;
    fn from_index(i: usize) -> Option<Self>;
    fn own(this: &Self::Ref<'_>) -> Self;
    fn first() -> Option<Self>;
    fn last() -> Option<Self>;
    fn prev(this: &Self::Ref<'_>) -> Option<Self>;
    fn succ(this: &Self::Ref<'_>) -> Option<Self>;
    fn count_from(from: &Self::Ref<'_>) -> Option<NonZeroUsize>;
    fn fold_each<B, F>(init: B, f: F) -> B
    where
        F: FnMut(B, Self) -> B;
    fn fold_each_from<B, F>(from: &Self::Ref<'_>, init: B, f: F) -> B
    where
        F: FnMut(B, Self) -> B;
}

impl GEnum for Zero {
    const CARD: Option<usize> = Some(0);

    fn to_index(this: &Self::Ref<'_>) -> Option<usize> {
        match *this {}
    }

    fn from_index(_i: usize) -> Option<Self> {
        None
    }

    fn own(this: &Self::Ref<'_>) -> Self {
        match *this {}
    }

    fn first() -> Option<Self> {
        None
    }

    fn last() -> Option<Self> {
        None
    }

    fn prev(this: &Self::Ref<'_>) -> Option<Self> {
        match *this {}
    }

    fn succ(this: &Self::Ref<'_>) -> Option<Self> {
        match *this {}
    }

    fn count_from(from: &Self::Ref<'_>) -> Option<NonZeroUsize> {
        match *from {}
    }

    fn fold_each<B, F>(init: B, _f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        init
    }

    fn fold_each_from<B, F>(from: &Self::Ref<'_>, _init: B, _f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        match *from {}
    }
}

impl GEnum for One {
    const CARD: Option<usize> = Some(1);

    fn to_index(_this: &Self::Ref<'_>) -> Option<usize> {
        Some(0)
    }

    fn from_index(i: usize) -> Option<Self> {
        (i == 0).then_some(One)
    }

    fn own(_this: &Self::Ref<'_>) -> Self {
        One
    }

    fn first() -> Option<Self> {
        Some(One)
    }

    fn last() -> Option<Self> {
        Some(One)
    }

    fn prev(_this: &Self::Ref<'_>) -> Option<Self> {
        None
    }

    fn succ(_this: &Self::Ref<'_>) -> Option<Self> {
        None
    }

    fn count_from(_from: &Self::Ref<'_>) -> Option<NonZeroUsize> {
        Some(1.try_into().unwrap())
    }

    fn fold_each<B, F>(init: B, mut f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        f(init, One)
    }

    fn fold_each_from<B, F>(_from: &Self::Ref<'_>, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        f(init, One)
    }
}

impl<T: GEnum, R: GEnum> GEnum for Sum<T, R> {
    const CARD: Option<usize> = match (T::CARD, R::CARD) {
        (Some(x), Some(y)) => x.checked_add(y),
        _ => None,
    };

    fn to_index(this: &Self::Ref<'_>) -> Option<usize> {
        match this {
            Sum::This(x) => T::to_index(x),
            Sum::Next(x) => T::CARD?.checked_add(R::to_index(x)?),
        }
    }

    fn from_index(i: usize) -> Option<Self> {
        match T::CARD {
            Some(c) if i >= c => R::from_index(i - c).map(Sum::Next),
            _ => T::from_index(i).map(Sum::This),
        }
    }

    fn own(this: &Self::Ref<'_>) -> Self {
        match this {
            Sum::This(x) => Sum::This(T::own(x)),
            Sum::Next(x) => Sum::Next(R::own(x)),
        }
    }

    fn first() -> Option<Self> {
        T::first().map(Sum::This).or(R::first().map(Sum::Next))
    }

    fn last() -> Option<Self> {
        R::last().map(Sum::Next).or(T::last().map(Sum::This))
    }

    fn prev(this: &Self::Ref<'_>) -> Option<Self> {
        match this {
            Sum::This(x) => T::prev(x).map(Sum::This),
            Sum::Next(x) => match R::prev(x) {
                Some(x) => Some(Sum::Next(x)),
                None => T::last().map(Sum::This),
            },
        }
    }

    fn succ(this: &Self::Ref<'_>) -> Option<Self> {
        match this {
            Sum::Next(x) => R::succ(x).map(Sum::Next),
            Sum::This(x) => match T::succ(x) {
                Some(x) => Some(Sum::This(x)),
                None => R::first().map(Sum::Next),
            },
        }
    }

    fn count_from(from: &Self::Ref<'_>) -> Option<NonZeroUsize> {
        match from {
            Sum::This(x) => T::count_from(x)?.checked_add(R::CARD?),
            Sum::Next(x) => R::count_from(x),
        }
    }

    fn fold_each<B, F>(init: B, mut f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        let init = T::fold_each(init, |acc, x| f(acc, Sum::This(x)));
        R::fold_each(init, |acc, x| f(acc, Sum::Next(x)))
    }

    fn fold_each_from<B, F>(from: &Self::Ref<'_>, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        match from {
            Sum::This(from) => {
                let init = T::fold_each_from(from, init, |acc, x| f(acc, Sum::This(x)));
                R::fold_each(init, |acc, x| f(acc, Sum::Next(x)))
            }
            Sum::Next(from) => R::fold_each_from(from, init, |acc, x| f(acc, Sum::Next(x))),
        }
    }
}

impl<T: Enum, R: GEnum> GEnum for Product<T, R> {
    const CARD: Option<usize> = match (T::CARD, R::CARD) {
        (Some(x), Some(y)) => x.checked_mul(y),
        _ => None,
    };

    fn to_index(this: &Self::Ref<'_>) -> Option<usize> {
        let Product(x, y) = this;
        match x.to_index()? {
            0 => R::to_index(y),
            x => x.checked_mul(R::CARD?)?.checked_add(R::to_index(y)?),
        }
    }

    fn from_index(i: usize) -> Option<Self> {
        match R::CARD {
            Some(0) => None,
            Some(c) => Some(Product(T::from_index(i / c)?, R::from_index(i % c)?)),
            None => Some(Product(T::from_index(0)?, R::from_index(i)?)),
        }
    }

    fn own(this: &Self::Ref<'_>) -> Self {
        Product(this.0.clone(), R::own(&this.1))
    }

    fn first() -> Option<Self> {
        Some(Product(T::first()?, R::first()?))
    }

    fn last() -> Option<Self> {
        Some(Product(T::last()?, R::last()?))
    }

    fn prev(this: &Self::Ref<'_>) -> Option<Self> {
        Some(match R::prev(&this.1) {
            Some(x) => Product(this.0.clone(), x),
            None => Product(this.0.prev()?, R::last()?),
        })
    }

    fn succ(this: &Self::Ref<'_>) -> Option<Self> {
        Some(match R::succ(&this.1) {
            Some(x) => Product(this.0.clone(), x),
            None => Product(this.0.succ()?, R::first()?),
        })
    }

    fn count_from(from: &Self::Ref<'_>) -> Option<NonZeroUsize> {
        let more = match T::count_from(from.0)?.get() {
            0 => unreachable!(),
            1 => 0,
            c => (c - 1).checked_mul(R::CARD?)?,
        };
        R::count_from(&from.1)?.checked_add(more)
    }

    fn fold_each<B, F>(init: B, mut f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        T::fold_each(init, |init, l| {
            R::fold_each(init, |acc, r| f(acc, Product(l.clone(), r)))
        })
    }

    fn fold_each_from<B, F>(from: &Self::Ref<'_>, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        let init = R::fold_each_from(&from.1, init, |acc, r| f(acc, Product(from.0.clone(), r)));
        if let Some(from) = from.0.succ() {
            T::fold_each_from(&from, init, |init, l| {
                R::fold_each(init, |acc, r| f(acc, Product(l.clone(), r)))
            })
        } else {
            init
        }
    }
}

/// 基于 [`Generic`] 自动实现
///
/// 若某类型实现了 [`Generic`] 和 [`GenericEnum`]，将会自动实现 [`Enum`]。
pub trait GenericEnum: Generic + Clone
where
    Self::Repr: GEnum,
{
}

impl<T> Enum for T
where
    T: GenericEnum + Clone,
    T::Repr: GEnum,
{
    const CARD: Option<usize> = T::Repr::CARD;

    fn to_index(&self) -> Option<usize> {
        T::Repr::to_index(&self.as_repr())
    }

    fn from_index(i: usize) -> Option<Self> {
        T::Repr::from_index(i).map(T::from_repr)
    }

    fn first() -> Option<Self> {
        T::Repr::first().map(T::from_repr)
    }

    fn last() -> Option<Self> {
        T::Repr::last().map(T::from_repr)
    }

    fn prev(&self) -> Option<Self> {
        T::Repr::prev(&self.as_repr()).map(T::from_repr)
    }

    fn succ(&self) -> Option<Self> {
        T::Repr::succ(&self.as_repr()).map(T::from_repr)
    }

    fn count_from(from: &Self) -> Option<NonZeroUsize> {
        T::Repr::count_from(&from.as_repr())
    }

    fn fold_each<B, F>(init: B, mut f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        T::Repr::fold_each(init, |acc, x| f(acc, T::from_repr(x)))
    }

    fn fold_each_from<B, F>(from: &Self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self) -> B,
    {
        T::Repr::fold_each_from(&from.as_repr(), init, |acc, x| f(acc, T::from_repr(x)))
    }
}
