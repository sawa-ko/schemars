use crate::prelude::*;
use arrayvec07::{ArrayString, ArrayVec};

#[test]
fn arrayvec07() {
    test!(ArrayVec<i32, 8>)
        .assert_snapshot()
        .assert_allows_serde_roundtrip([
            ArrayVec::from_iter([]),
            ArrayVec::from_iter([1, 2, 3, 4, 5, 6, 7, 8]),
        ])
        .assert_rejects([json!([1, 2, 3, 4, 5, 6, 7, 8, 9])])
        .assert_matches_deserialize(arbitrary_values());
}

#[test]
fn arrayvec07_arraystring() {
    test!(ArrayString<8>)
        .assert_identical::<String>()
        .assert_allows_serde_roundtrip(["".try_into().unwrap(), "12345678".try_into().unwrap()])
        // There's not a good way to express UTF-8 byte length in JSON schema, so the generated schema
        // just ignores the ArrayString's capacity. This means we unfortunately can't do:
        // .assert_rejects(["12345678".try_into().unwrap()]);
        .assert_matches_deserialize(arbitrary_nonstring_values());
}
