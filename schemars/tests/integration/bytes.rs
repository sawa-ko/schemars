use crate::prelude::*;
use bytes1::{Bytes, BytesMut};

#[test]
fn bytes() {
    test!(Bytes)
        .assert_snapshot()
        .assert_allows_serde_roundtrip([Bytes::new(), Bytes::from_iter([12; 34])])
        .assert_matches_deserialize(arbitrary_values());
}

#[test]
fn bytes_mut() {
    test!(BytesMut)
        .assert_identical::<Bytes>()
        .assert_allows_serde_roundtrip([BytesMut::new(), BytesMut::from_iter([12; 34])])
        .assert_matches_deserialize(arbitrary_values());
}
