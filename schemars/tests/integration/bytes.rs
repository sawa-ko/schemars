use crate::prelude::*;
use bytes1::{Bytes, BytesMut};

#[test]
fn bytes() {
    test!(Bytes)
        .assert_snapshot()
        .assert_allows_ser_roundtrip([Bytes::new(), Bytes::from_iter([12; 34])])
        // FIXME schema allows out-of-range positive integers
        .assert_matches_de_roundtrip(arbitrary_values().filter(|v| !is_array_of_u64(v)));
}

#[test]
fn bytes_mut() {
    test!(BytesMut)
        .assert_identical::<Bytes>()
        .assert_allows_ser_roundtrip([BytesMut::new(), BytesMut::from_iter([12; 34])])
        // FIXME schema allows out-of-range positive integers
        .assert_matches_de_roundtrip(arbitrary_values().filter(|v| !is_array_of_u64(v)));
}

fn is_array_of_u64(value: &Value) -> bool {
    value
        .as_array()
        .is_some_and(|a| a.iter().all(Value::is_u64))
}
