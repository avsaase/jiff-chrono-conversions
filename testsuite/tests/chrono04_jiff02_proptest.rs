use chrono_04 as chrono;
use chrono_tz_04 as chrono_tz;
use jiff_02 as jiff;
use jiff_chrono_conversions::{ToChrono, ToJiff, TryToChrono, TryToJiff};
use proptest::prelude::*;
use proptest_arbitrary_interop::arb;

use chrono::{Datelike, Timelike};

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

fn chrono_utc_datetime_leap_second_in_jiff_range()
-> impl Strategy<Value = chrono::DateTime<chrono::Utc>> {
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
    fn date_chrono_to_jiff_matches_components(date in chrono_date_in_jiff_range()) {
        let jiff_date = date.to_jiff().expect("date is within jiff's range");
        prop_assert_eq!(jiff_date.year() as i32, date.year());
        prop_assert_eq!(jiff_date.month() as u32, date.month());
        prop_assert_eq!(jiff_date.day() as u32, date.day());
    }

    #[test]
    fn date_jiff_to_chrono_matches_components(date in arb::<jiff::civil::Date>()) {
        let chrono_date = date.to_chrono();
        prop_assert_eq!(chrono_date.year(), date.year() as i32);
        prop_assert_eq!(chrono_date.month(), date.month() as u32);
        prop_assert_eq!(chrono_date.day(), date.day() as u32);
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
    fn time_chrono_to_jiff_matches_components(
        time in arb::<chrono::NaiveTime>().prop_filter("not a leap second", |t| t.nanosecond() < 1_000_000_000)
    ) {
        let jiff_time = time.to_jiff();
        prop_assert_eq!(jiff_time.hour() as u32, time.hour());
        prop_assert_eq!(jiff_time.minute() as u32, time.minute());
        prop_assert_eq!(jiff_time.second() as u32, time.second());
        prop_assert_eq!(jiff_time.subsec_nanosecond() as u32, time.nanosecond());
    }

    #[test]
    fn time_jiff_to_chrono_matches_components(time in arb::<jiff::civil::Time>()) {
        let chrono_time = time.to_chrono();
        prop_assert_eq!(chrono_time.hour() as i8, time.hour());
        prop_assert_eq!(chrono_time.minute() as i8, time.minute());
        prop_assert_eq!(chrono_time.second() as i8, time.second());
        prop_assert_eq!(chrono_time.nanosecond() as i32, time.subsec_nanosecond());
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

    #[test]
    fn datetime_chrono_to_jiff_matches_components(
        date in chrono_date_in_jiff_range(),
        time in arb::<chrono::NaiveTime>().prop_filter("not a leap second", |t| t.nanosecond() < 1_000_000_000),
    ) {
        let datetime = chrono::NaiveDateTime::new(date, time);
        let jiff_datetime = datetime.to_jiff().expect("datetime is within jiff's range");
        prop_assert_eq!(jiff_datetime.date().year() as i32, date.year());
        prop_assert_eq!(jiff_datetime.date().month() as u32, date.month());
        prop_assert_eq!(jiff_datetime.date().day() as u32, date.day());
        prop_assert_eq!(jiff_datetime.time().hour() as u32, time.hour());
        prop_assert_eq!(jiff_datetime.time().minute() as u32, time.minute());
        prop_assert_eq!(jiff_datetime.time().second() as u32, time.second());
        prop_assert_eq!(jiff_datetime.time().subsec_nanosecond() as u32, time.nanosecond());
    }

    #[test]
    fn datetime_jiff_to_chrono_matches_components(datetime in arb::<jiff::civil::DateTime>()) {
        let chrono_datetime = datetime.to_chrono();
        prop_assert_eq!(chrono_datetime.date().year(), datetime.date().year() as i32);
        prop_assert_eq!(chrono_datetime.date().month(), datetime.date().month() as u32);
        prop_assert_eq!(chrono_datetime.date().day(), datetime.date().day() as u32);
        prop_assert_eq!(chrono_datetime.time().hour() as i8, datetime.time().hour());
        prop_assert_eq!(chrono_datetime.time().minute() as i8, datetime.time().minute());
        prop_assert_eq!(chrono_datetime.time().second() as i8, datetime.time().second());
        prop_assert_eq!(
            chrono_datetime.time().nanosecond() as i32,
            datetime.time().subsec_nanosecond()
        );
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

    // Compare the total nanosecond instant rather than the individual `second`/`nanosecond`
    // fields directly: `jiff::Timestamp` ties their signs together (see the leap-second test
    // above), so for pre-epoch instants the two libraries don't necessarily split the same
    // instant across the fields the same way, even though they represent the same instant.
    #[test]
    fn timestamp_chrono_to_jiff_matches_components(chrono_dt in chrono_utc_datetime_in_jiff_range()) {
        let jiff_ts = chrono_dt.to_jiff().expect("timestamp is within jiff's range");
        let actual =
            jiff_ts.as_second() as i128 * 1_000_000_000 + jiff_ts.subsec_nanosecond() as i128;
        let expected =
            chrono_dt.timestamp() as i128 * 1_000_000_000 + chrono_dt.timestamp_subsec_nanos() as i128;
        prop_assert_eq!(actual, expected);
    }

    #[test]
    fn timestamp_jiff_to_chrono_matches_components(jiff_ts in arb::<jiff::Timestamp>()) {
        let chrono_dt = jiff_ts.to_chrono();
        let actual =
            chrono_dt.timestamp() as i128 * 1_000_000_000 + chrono_dt.timestamp_subsec_nanos() as i128;
        let expected =
            jiff_ts.as_second() as i128 * 1_000_000_000 + jiff_ts.subsec_nanosecond() as i128;
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

    #[test]
    fn offset_chrono_to_jiff_matches_components(chrono_offset in arb::<chrono::FixedOffset>()) {
        let jiff_offset = chrono_offset.to_jiff();
        prop_assert_eq!(jiff_offset.seconds(), chrono_offset.local_minus_utc());
    }

    #[test]
    fn offset_jiff_to_chrono_matches_components(jiff_offset in jiff_offset_in_chrono_range()) {
        let chrono_offset = jiff_offset.to_chrono().expect("offset is within chrono's range");
        prop_assert_eq!(chrono_offset.local_minus_utc(), jiff_offset.seconds());
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

    #[test]
    fn zoned_chrono_to_jiff_matches_components(chrono_dt in chrono_fixedoffset_datetime_in_jiff_range()) {
        let zoned = chrono_dt.to_jiff().expect("value is within jiff's range");
        let utc = chrono_dt.with_timezone(&chrono::Utc);
        let actual = zoned.timestamp().as_second() as i128 * 1_000_000_000
            + zoned.timestamp().subsec_nanosecond() as i128;
        let expected = utc.timestamp() as i128 * 1_000_000_000 + utc.timestamp_subsec_nanos() as i128;
        prop_assert_eq!(actual, expected);
        prop_assert_eq!(zoned.offset().seconds(), chrono_dt.offset().local_minus_utc());
    }

    #[test]
    fn zoned_jiff_to_chrono_matches_components(
        timestamp in arb::<jiff::Timestamp>(),
        offset in jiff_offset_in_chrono_range(),
    ) {
        let zoned = timestamp.to_zoned(jiff::tz::TimeZone::fixed(offset));
        let chrono_dt = zoned.to_chrono().expect("offset is within chrono's range");
        let utc = chrono_dt.with_timezone(&chrono::Utc);
        let actual = utc.timestamp() as i128 * 1_000_000_000 + utc.timestamp_subsec_nanos() as i128;
        let expected =
            timestamp.as_second() as i128 * 1_000_000_000 + timestamp.subsec_nanosecond() as i128;
        prop_assert_eq!(actual, expected);
        prop_assert_eq!(chrono_dt.offset().local_minus_utc(), offset.seconds());
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
