#![no_main]
use libfuzzer_sys::fuzz_target;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq)]
enum PlainEnum {
    A,
    B,
    C,
    D,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq)]
enum Enum {
    A(u8),
    B(()),
    C(Vec<PlainEnum>),
    D(i128),
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq)]
enum FloatEnum {
    A(Enum),
    E(Option<f32>),
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq)]
struct Struct {
    _a: (),
    _b: u8,
    _c: Vec<Enum>,
    _d: (u128, i8, (), PlainEnum, String),
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq)]
struct FloatStruct {
    _a: Struct,
    _b: f64,
}

macro_rules! round_trip {
    ($ty:ty, $data:ident, $equality:expr) => {{
        #[cfg(feature = "debug")]
        println!("roundtripping {}", stringify!($ty));

        let x: Result<$ty, _> = <$ty as BorshDeserialize>::try_from_slice($data);
        if let Ok(inner) = x {
            #[cfg(feature = "debug")]
            dbg!(&inner);

            let ser = <$ty as BorshSerialize>::try_to_vec(&inner)
                .expect("a deserialized type should serialize");
            #[cfg(feature = "debug")]
            dbg!(&ser);

            let des: $ty = <$ty as BorshDeserialize>::try_from_slice(&ser)
                .expect("a serialized type should deserialize");
            #[cfg(feature = "debug")]
            dbg!(&des);

            if $equality {
                assert_eq!(inner, des, "roundtripped object changed");
            }
        }
    }};
}

macro_rules! from_bytes {
    ($ty:ty, $data:ident, $equality:expr) => {{
        round_trip!($ty, $data, $equality);
        round_trip!(Vec<$ty>, $data, $equality);
        round_trip!(Option<$ty>, $data, $equality);
    }};
}

fuzz_target!(|data: &[u8]| {
    from_bytes!(bool, data, true);
    from_bytes!(i8, data, true);
    from_bytes!(i16, data, true);
    from_bytes!(i32, data, true);
    from_bytes!(i64, data, true);
    from_bytes!(i128, data, true);
    from_bytes!(u8, data, true);
    from_bytes!(u16, data, true);
    from_bytes!(u32, data, true);
    from_bytes!(u64, data, true);
    from_bytes!(u128, data, true);
    from_bytes!(f32, data, false);
    from_bytes!(f64, data, false);
    from_bytes!(String, data, true);
    from_bytes!((), data, true);
    from_bytes!(PlainEnum, data, true);
    from_bytes!(Enum, data, true);
    from_bytes!(FloatEnum, data, false);
    from_bytes!(Struct, data, true);
    from_bytes!(FloatStruct, data, false);
});
