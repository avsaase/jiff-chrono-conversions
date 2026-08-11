#![cfg(all(feature = "chrono04", feature = "jiff02"))]

use chrono_04 as chrono;
use chrono_tz_04 as chrono_tz;
use jiff_02 as jiff;
use jiff_chrono_conversions::{ToChrono, ToJiff, TryToChrono, TryToJiff};
use proptest::prelude::*;
use proptest_arbitrary_interop::arb;

use chrono::{Datelike, Timelike};

/* Strategies */
//
// `chrono` and `jiff` both implement `arbitrary::Arbitrary` for their types (gated behind an
// `arbitrary` feature), generating values across each type's *own* full valid range. We use
// `proptest_arbitrary_interop::arb` to turn those `Arbitrary` impls into proptest strategies, and
// then filter down to the overlapping range where needed, rather than hand-constructing values
// ourselves.

fn jiff_year_range() -> std::ops::RangeInclusive<i32> {
    (jiff::civil::Date::MIN.year() as i32)..=(jiff::civil::Date::MAX.year() as i32)
}

fn chrono_date_in_jiff_range() -> impl Strategy<Value = chrono::NaiveDate> {
    let years = jiff_year_range();
    arb::<chrono::NaiveDate>().prop_filter("date within jiff's date range", move |d| {
        years.contains(&d.year())
    })
}

fn chrono_date_outside_jiff_range() -> impl Strategy<Value = chrono::NaiveDate> {
    let years = jiff_year_range();
    arb::<chrono::NaiveDate>().prop_filter("date outside jiff's date range", move |d| {
        !years.contains(&d.year())
    })
}

fn timestamp_seconds_in_jiff_range() -> std::ops::RangeInclusive<i64> {
    jiff::Timestamp::MIN.as_second()..=jiff::Timestamp::MAX.as_second()
}

fn chrono_utc_datetime_in_jiff_range() -> impl Strategy<Value = chrono::DateTime<chrono::Utc>> {
    let seconds = timestamp_seconds_in_jiff_range();
    arb::<chrono::DateTime<chrono::Utc>>().prop_filter(
        "timestamp within jiff's range, not a leap second",
        move |dt| seconds.contains(&dt.timestamp()) && dt.timestamp_subsec_nanos() < 1_000_000_000,
    )
}

fn chrono_utc_datetime_leap_second_in_jiff_range(
) -> impl Strategy<Value = chrono::DateTime<chrono::Utc>> {
    // Leap seconds are a vanishingly small fraction of what `arb::<chrono::NaiveTime>()`
    // generates, so rather than filtering `arb::<chrono::DateTime<chrono::Utc>>()` down to them
    // (which the proptest rejection sampler can't do fast enough), build one directly: an
    // in-range date paired with a `NaiveTime` whose second is deliberately 59 with the
    // leap-second nanosecond overflow.
    let seconds = timestamp_seconds_in_jiff_range();
    (
        chrono_date_in_jiff_range(),
        0u32..24,
        0u32..60,
        1_000_000_000u32..2_000_000_000,
    )
        .prop_map(|(date, hour, minute, nanos)| {
            let time = chrono::NaiveTime::from_hms_nano_opt(hour, minute, 59, nanos).unwrap();
            chrono::DateTime::from_naive_utc_and_offset(
                chrono::NaiveDateTime::new(date, time),
                chrono::Utc,
            )
        })
        .prop_filter("timestamp within jiff's range", move |dt| {
            seconds.contains(&dt.timestamp())
        })
}

fn chrono_utc_datetime_outside_jiff_range() -> impl Strategy<Value = chrono::DateTime<chrono::Utc>>
{
    let seconds = timestamp_seconds_in_jiff_range();
    arb::<chrono::DateTime<chrono::Utc>>()
        .prop_filter("timestamp outside jiff's range", move |dt| {
            !seconds.contains(&dt.timestamp())
        })
}

fn chrono_offset_seconds_range() -> std::ops::RangeInclusive<i32> {
    -86_399..=86_399
}

fn jiff_offset_in_chrono_range() -> impl Strategy<Value = jiff::tz::Offset> {
    let seconds = chrono_offset_seconds_range();
    arb::<jiff::tz::Offset>().prop_filter("offset within chrono's range", move |o| {
        seconds.contains(&o.seconds())
    })
}

fn jiff_offset_outside_chrono_range() -> impl Strategy<Value = jiff::tz::Offset> {
    let seconds = chrono_offset_seconds_range();
    arb::<jiff::tz::Offset>().prop_filter("offset outside chrono's range", move |o| {
        !seconds.contains(&o.seconds())
    })
}

fn chrono_fixedoffset_datetime_in_jiff_range()
-> impl Strategy<Value = chrono::DateTime<chrono::FixedOffset>> {
    let seconds = timestamp_seconds_in_jiff_range();
    arb::<chrono::DateTime<chrono::FixedOffset>>().prop_filter(
        "timestamp within jiff's range, not a leap second",
        move |dt| seconds.contains(&dt.timestamp()) && dt.timestamp_subsec_nanos() < 1_000_000_000,
    )
}

/* Date */

proptest! {
    #[test]
    fn date_roundtrip_chrono_to_jiff_to_chrono(date in chrono_date_in_jiff_range()) {
        let jiff_date = date.to_jiff().expect("date is within jiff's range");
        prop_assert_eq!(jiff_date.to_chrono(), date);
    }

    #[test]
    fn date_roundtrip_jiff_to_chrono_to_jiff(date in arb::<jiff::civil::Date>()) {
        let chrono_date = date.to_chrono();
        prop_assert_eq!(chrono_date.to_jiff().expect("date came from jiff's range"), date);
    }

    #[test]
    fn date_outside_jiff_range_fails_to_convert(date in chrono_date_outside_jiff_range()) {
        prop_assert!(date.to_jiff().is_err());
    }
}

/* Time */

proptest! {
    #[test]
    fn time_roundtrip_chrono_to_jiff_to_chrono(
        time in arb::<chrono::NaiveTime>().prop_filter("not a leap second", |t| t.nanosecond() < 1_000_000_000)
    ) {
        prop_assert_eq!(time.to_jiff().to_chrono(), time);
    }

    #[test]
    fn time_roundtrip_jiff_to_chrono_to_jiff(time in arb::<jiff::civil::Time>()) {
        prop_assert_eq!(time.to_chrono().to_jiff(), time);
    }

    #[test]
    fn time_leap_second_normalizes_to_last_second_of_minute(
        time in arb::<chrono::NaiveTime>().prop_filter("leap second", |t| t.nanosecond() >= 1_000_000_000)
    ) {
        let jiff_time = time.to_jiff();
        prop_assert_eq!(jiff_time.hour(), time.hour() as i8);
        prop_assert_eq!(jiff_time.minute(), time.minute() as i8);
        prop_assert_eq!(jiff_time.second(), 59);
        prop_assert_eq!(jiff_time.subsec_nanosecond(), 999_999_999);
    }
}

/* DateTime */

proptest! {
    #[test]
    fn datetime_roundtrip_chrono_to_jiff_to_chrono(
        date in chrono_date_in_jiff_range(),
        time in arb::<chrono::NaiveTime>().prop_filter("not a leap second", |t| t.nanosecond() < 1_000_000_000),
    ) {
        let datetime = chrono::NaiveDateTime::new(date, time);
        let jiff_datetime = datetime.to_jiff().expect("datetime is within jiff's range");
        prop_assert_eq!(jiff_datetime.to_chrono(), datetime);
    }

    #[test]
    fn datetime_roundtrip_jiff_to_chrono_to_jiff(datetime in arb::<jiff::civil::DateTime>()) {
        let chrono_datetime = datetime.to_chrono();
        prop_assert_eq!(
            chrono_datetime.to_jiff().expect("datetime came from jiff's range"),
            datetime
        );
    }

    #[test]
    fn datetime_outside_jiff_range_fails_to_convert(
        date in chrono_date_outside_jiff_range(),
        time in arb::<chrono::NaiveTime>(),
    ) {
        let datetime = chrono::NaiveDateTime::new(date, time);
        prop_assert!(datetime.to_jiff().is_err());
    }
}

/* Timestamp */

proptest! {
    #[test]
    fn timestamp_roundtrip_chrono_to_jiff_to_chrono(chrono_dt in chrono_utc_datetime_in_jiff_range()) {
        let jiff_ts = chrono_dt.to_jiff().expect("timestamp is within jiff's range");
        prop_assert_eq!(jiff_ts.to_chrono(), chrono_dt);
    }

    #[test]
    fn timestamp_roundtrip_jiff_to_chrono_to_jiff(jiff_ts in arb::<jiff::Timestamp>()) {
        let chrono_dt = jiff_ts.to_chrono();
        prop_assert_eq!(chrono_dt.to_jiff().expect("timestamp came from jiff's range"), jiff_ts);
    }

    #[test]
    fn timestamp_outside_jiff_range_fails_to_convert(chrono_dt in chrono_utc_datetime_outside_jiff_range()) {
        prop_assert!(chrono_dt.to_jiff().is_err());
    }

    #[test]
    fn timestamp_leap_second_normalizes_to_last_nanosecond_of_second(
        chrono_dt in chrono_utc_datetime_leap_second_in_jiff_range()
    ) {
        let jiff_ts = chrono_dt.to_jiff().expect("timestamp is within jiff's range");
        // `jiff::Timestamp::new` keeps the sign of its `second` and `nanosecond` components
        // aligned, so for pre-epoch leap seconds the carry ends up split across both fields
        // differently than for post-epoch ones. Compare the total nanosecond instant instead of
        // the individual fields to check the same "last nanosecond of the second" invariant
        // regardless of that internal normalization.
        let actual =
            jiff_ts.as_second() as i128 * 1_000_000_000 + jiff_ts.subsec_nanosecond() as i128;
        let expected = chrono_dt.timestamp() as i128 * 1_000_000_000 + 999_999_999;
        prop_assert_eq!(actual, expected);
    }
}

/* Offset */

proptest! {
    #[test]
    fn offset_roundtrip_chrono_to_jiff_to_chrono(chrono_offset in arb::<chrono::FixedOffset>()) {
        let jiff_offset = chrono_offset.to_jiff();
        prop_assert_eq!(
            jiff_offset.to_chrono().expect("offset came from chrono's range"),
            chrono_offset
        );
    }

    #[test]
    fn offset_roundtrip_jiff_to_chrono_to_jiff(jiff_offset in jiff_offset_in_chrono_range()) {
        let chrono_offset = jiff_offset.to_chrono().expect("offset is within chrono's range");
        prop_assert_eq!(chrono_offset.to_jiff(), jiff_offset);
    }

    #[test]
    fn offset_outside_chrono_range_fails_to_convert(jiff_offset in jiff_offset_outside_chrono_range()) {
        prop_assert!(jiff_offset.to_chrono().is_err());
    }
}

/* DateTime with time zone */

proptest! {
    #[test]
    fn zoned_roundtrip_chrono_to_jiff_to_chrono(chrono_dt in chrono_fixedoffset_datetime_in_jiff_range()) {
        let zoned = chrono_dt.to_jiff().expect("value is within jiff's range");
        prop_assert_eq!(
            zoned.to_chrono().expect("offset is within chrono's range"),
            chrono_dt
        );
    }

    #[test]
    fn zoned_roundtrip_jiff_to_chrono_to_jiff(
        timestamp in arb::<jiff::Timestamp>(),
        offset in jiff_offset_in_chrono_range(),
    ) {
        let zoned = timestamp.to_zoned(jiff::tz::TimeZone::fixed(offset));
        let chrono_dt = zoned.to_chrono().expect("offset is within chrono's range");
        let roundtripped = chrono_dt.to_jiff().expect("value came from jiff's range");
        prop_assert_eq!(roundtripped.timestamp(), zoned.timestamp());
        prop_assert_eq!(roundtripped.offset(), zoned.offset());
    }

    #[test]
    fn zoned_with_offset_outside_chrono_range_fails_to_convert(
        timestamp in arb::<jiff::Timestamp>(),
        offset in jiff_offset_outside_chrono_range(),
    ) {
        let zoned = timestamp.to_zoned(jiff::tz::TimeZone::fixed(offset));
        prop_assert!(zoned.to_chrono().is_err());
    }
}

/* Time zone */

proptest! {
    #[test]
    fn timezone_roundtrip_for_shared_iana_names(
        tz in proptest::sample::select(chrono_tz::TZ_VARIANTS.as_slice())
    ) {
        // Not every IANA name in `chrono-tz`'s database is guaranteed to also be present in
        // `jiff`'s time zone database (and vice versa), so we only assert the roundtrip for
        // names that both crates recognize.
        if let Ok(jiff_tz) = jiff::tz::TimeZone::get(tz.name()) {
            let back = jiff_tz.to_chrono().expect("name came from chrono-tz's own database");
            prop_assert_eq!(back, tz);
        }
    }
}
