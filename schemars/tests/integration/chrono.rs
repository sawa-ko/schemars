use crate::prelude::*;
use chrono04::prelude::*;

#[derive(JsonSchema, Serialize, Deserialize)]
struct ChronoTypes {
    weekday: Weekday,
    date_time: DateTime<Utc>,
    naive_date: NaiveDate,
    naive_date_time: NaiveDateTime,
    naive_time: NaiveTime,
}

#[test]
fn chrono() {
    test!(ChronoTypes).assert_snapshot();

    test!(Weekday)
        .assert_allows_serde_roundtrip([Weekday::Mon])
        .assert_matches_deserialize(arbitrary_values());

    test!(DateTime<Utc>)
        .assert_allows_serde_roundtrip_default()
        .assert_matches_deserialize(arbitrary_values());

    test!(NaiveDate)
        .assert_allows_serde_roundtrip_default()
        .assert_matches_deserialize(arbitrary_values());

    test!(NaiveDateTime)
        .assert_allows_serde_roundtrip_default()
        // Custom format "partial-date-time", so arbitrary strings technically allowed by schema
        .assert_matches_deserialize(arbitrary_nonstring_values());

    test!(NaiveTime)
        .assert_allows_serde_roundtrip_default()
        // Custom format "partial-time", so arbitrary strings technically allowed by schema
        .assert_matches_deserialize(arbitrary_nonstring_values());
}
