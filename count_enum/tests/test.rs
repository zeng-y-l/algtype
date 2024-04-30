use algtype::Generic;
use count_enum::{Enum, GenericEnum};
use proptest::prelude::*;
use proptest_derive::Arbitrary;
use std::{fmt::Debug, num::NonZeroUsize};

// 不知为何，rust-analyzer会报错
#[derive(Clone, Generic, Debug, PartialEq, Arbitrary)]
enum Ty<T> {
    A,
    B,
    C(Option<i8>),
    D { a: T, b: bool },
    E(Result<(bool, bool, ()), T>),
}

impl<T: Enum> GenericEnum for Ty<T> {}

fn assert_enum<T: Enum + PartialEq + Debug + Arbitrary>() {
    let min = T::first().unwrap();
    let max = T::last().unwrap();
    assert_eq!(T::count_from(&min).map(NonZeroUsize::get), T::CARD);
    assert_eq!(T::count_from(&max), Some(1.try_into().unwrap()));
    assert_eq!(min.prev(), None);
    assert_eq!(max.succ(), None);
    assert_eq!(min.to_index(), Some(0));
    assert_eq!(max.to_index().and_then(|x| x.checked_add(1)), T::CARD);

    let test = |x: T| {
        if let Some(i) = x.to_index() {
            assert_eq!(Some(&x), T::from_index(i).as_ref());
            assert_eq!(i.checked_sub(1), x.prev().as_ref().and_then(T::to_index));
        } else {
            assert_eq!(None, x.succ().as_ref().and_then(T::to_index));
        }

        if let Some(y) = x.succ() {
            assert_eq!(y.prev().as_ref(), Some(&x));
        }
        if let Some(y) = x.prev() {
            assert_eq!(y.succ().as_ref(), Some(&x));
        }

        assert_eq!(
            x.to_index()
                .and_then(|i| T::count_from(&x)?.get().checked_add(i)),
            T::CARD
        );
    };
    proptest!(|(x: T)| test(x));
}

fn assert_enum_iter<T: Enum + PartialEq + Debug + Arbitrary>() {
    assert_eq!(
        T::fold_each(T::first(), |x, y| {
            assert_eq!(x, Some(y));
            x.unwrap().succ()
        }),
        None
    );

    let test = |x: T| {
        assert_eq!(
            T::fold_each_from(&x, Some(x.clone()), |x, y| {
                assert_eq!(x, Some(y));
                x.unwrap().succ()
            }),
            None
        )
    };
    proptest!(|(x: T)| test(x));
}

#[test]
fn test() {
    assert_enum::<Ty<bool>>();
    assert_enum_iter::<Ty<()>>();
    assert_enum::<Ty<Option<u32>>>();
    assert_enum::<Option<i128>>();
}
