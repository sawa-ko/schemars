use crate::prelude::*;
use arrayvec07::{ArrayString, ArrayVec};
use serde_json::Value;

#[test]
fn arrayvec07() {
    test!(ArrayVec<i32, 8>)
        .assert_snapshot()
        .assert_allows_serde_roundtrip([
            ArrayVec::from_iter([]),
            ArrayVec::from_iter([1, 2, 3, 4, 5, 6, 7, 8]),
        ])
        .assert_matches_deserialize(
            (0..16).map(|len| Value::Array((0..len).map(Value::from).collect())),
        )
        // FIXME schema allows out-of-range positive integers
        .assert_matches_deserialize(arbitrary_values().filter(|v| !is_array_of_u64(v)));
}

#[test]
fn arrayvec07_arraystring() {
    test!(ArrayString<8>)
        .assert_identical::<String>()
        .assert_allows_serde_roundtrip(["".try_into().unwrap(), "12345678".try_into().unwrap()])
        // There's not a good way to express UTF-8 byte length in JSON schema,
        // so the generated schema just ignores the ArrayString's capacity.
        .assert_matches_deserialize(arbitrary_nonstring_values());
}

fn is_array_of_u64(value: &Value) -> bool {
    value
        .as_array()
        .is_some_and(|a| a.iter().all(Value::is_u64))
}
