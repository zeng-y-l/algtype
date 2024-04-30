use algtype::{Generic, One, Product, Repr, Sum, Zero};
use proptest::{arbitrary::Arbitrary, proptest};
use proptest_derive::Arbitrary;
use std::fmt::Debug;

#[derive(Clone, Generic, Debug, PartialEq, Arbitrary)]
struct Unit;

#[derive(Clone, Generic, Debug, PartialEq, Arbitrary)]
struct Tuple<T>(T, Unit, Enum<T>);

#[derive(Clone, Generic, Debug, PartialEq, Arbitrary)]
struct Struct<T: Arbitrary, I: Iterator> {
    a: T,
    b: Unit,
    c: Enum<T>,
    d: I::Item,
}

#[derive(Clone, Generic, Debug, PartialEq, Arbitrary)]
enum Enum<T> {
    Unit,
    TupleUnit(),
    Tuple(T, Unit),
    StructUnit {},
    Struct { a: T, b: i32 },
}

#[derive(Clone, Generic, Debug, PartialEq)]
enum Empty {}

#[derive(Clone, Generic, Debug, PartialEq)]
enum Ref<'a> {
    No(Empty),
    Ref(&'a str),
}

fn check<T: Generic<Repr = R> + PartialEq + Debug + Clone + Arbitrary, R: Repr>()
where
    for<'a> R::Ref<'a>: PartialEq + Debug,
    for<'a> R::Mut<'a>: PartialEq + Debug,
{
    proptest!(|(x: T)| assert_refl(x));
}

fn assert_refl<T: Generic<Repr = R> + PartialEq + Debug + Clone, R: Repr>(mut x: T)
where
    for<'a> R::Ref<'a>: PartialEq + Debug,
    for<'a> R::Mut<'a>: PartialEq + Debug,
{
    let mut repr = x.clone().into_repr();
    assert_eq!(x.as_repr(), repr.as_ref());
    assert_eq!(x.as_mut_repr(), repr.as_mut_ref());
    assert_eq!(x, T::from_repr(repr));
}

#[test]
fn test() {
    check::<Unit, Sum<One, Zero>>();
    check::<Tuple<_>, Sum<Product<bool, Product<Unit, Product<Enum<bool>, One>>>, Zero>>();
    check::<
        Struct<_, std::ops::Range<_>>,
        Sum<Product<i32, Product<Unit, Product<Enum<i32>, Product<i32, One>>>>, Zero>,
    >();
    check::<
        Enum<()>,
        Sum<
            One,
            Sum<
                One,
                Sum<
                    Product<(), Product<Unit, One>>,
                    Sum<One, Sum<Product<(), Product<i32, One>>, Zero>>,
                >,
            >,
        >,
    >();

    assert_refl::<Ref, Sum<Product<Empty, One>, Sum<Product<&str, One>, Zero>>>(Ref::Ref(""));
}
