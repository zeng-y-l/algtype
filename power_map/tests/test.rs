use count_enum::Enum;
use power_map::{MapKey, TotalMap};
use proptest::{arbitrary::Arbitrary, proptest};

fn assert_map<T: Arbitrary + MapKey + Enum + PartialEq>() {
    let m = TotalMap::from_fn(Ok);
    let check = |keys: &[_]| {
        let mut m = m.clone();
        for (i, k) in keys.iter().enumerate() {
            m[k] = Err(i);
        }

        for (k, v) in m.as_ref().map_with_key(|k, _| k) {
            assert_eq!(k, v);
        }

        for (k, v) in m {
            match v {
                Ok(v) => assert_eq!(k, v),
                Err(v) => assert_eq!(k, keys[v]),
            }
        }
    };
    proptest!(|(k: [T; 10])| check(&k));
}

#[test]
fn test() {
    assert_map::<i8>();
    assert_map::<Result<u8, (bool, Option<[bool; 3]>)>>();
}
