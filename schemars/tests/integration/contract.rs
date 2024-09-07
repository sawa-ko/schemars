use crate::prelude::*;

#[derive(JsonSchema, Deserialize, Serialize)]
#[serde(rename_all(serialize = "SCREAMING-KEBAB-CASE"), deny_unknown_fields)]
struct StructDenyUnknownFields {
    #[serde(skip_deserializing)]
    read_only: bool,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    write_only: bool,
    #[serde(default)]
    default: bool,
    #[serde(skip_serializing_if = "core::ops::Not::not")]
    skip_serializing_if: bool,
    #[serde(rename(serialize = "ser_renamed", deserialize = "de_renamed"))]
    renamed: bool,
    option: Option<bool>,
}

#[derive(JsonSchema, Deserialize, Serialize)]
struct StructAllowUnknownFields {
    #[serde(flatten)]
    inner: StructDenyUnknownFields,
}

#[test]
fn struct_deny_unknown_fields() {
    test!(StructDenyUnknownFields)
        .assert_snapshot()
        .assert_allows_de_roundtrip([
            json!({ "write_only": false, "skip_serializing_if": false, "de_renamed": false }),
            json!({ "write_only": true, "skip_serializing_if": true, "de_renamed": true, "default": true }),
            json!({ "write_only": true, "skip_serializing_if": true, "de_renamed": true, "option": true }),
        ])
        .assert_rejects_de([
            json!({ "skip_serializing_if": false, "de_renamed": false }),
            json!({ "write_only": false, "de_renamed": false }),
            json!({ "write_only": false, "skip_serializing_if": false }),
            json!({ "write_only": true, "skip_serializing_if": true, "de_renamed": true, "unknown": true }),
        ])
        .assert_matches_de_roundtrip(arbitrary_values());
}

#[test]
fn struct_allow_unknown_fields() {
    test!(StructAllowUnknownFields)
        .assert_snapshot()
        .assert_allows_de_roundtrip([
            json!({ "write_only": false, "skip_serializing_if": false, "de_renamed": false }),
            json!({ "write_only": true, "skip_serializing_if": true, "de_renamed": true, "default": true }),
            json!({ "write_only": true, "skip_serializing_if": true, "de_renamed": true, "option": true }),
            json!({ "write_only": true, "skip_serializing_if": true, "de_renamed": true, "unknown": true }),
        ])
        .assert_rejects_de([
            json!({ "skip_serializing_if": false, "de_renamed": false }),
            json!({ "write_only": false, "de_renamed": false }),
            json!({ "write_only": false, "skip_serializing_if": false }),
        ])
        .assert_matches_de_roundtrip(arbitrary_values());
}
