use chrono_04 as chrono;
use chrono_tz_010 as chrono_tz;
use jiff_02 as jiff;
use jiff_chrono_conversions::{ToChrono, ToJiff, TryToChrono, TryToJiff};
use proptest::prelude::*;
use proptest_arbitrary_interop::arb;

use chrono::{Datelike, Timelike};

/* Date */

proptest! {
    #[test]
    fn date_roundtrip_chrono_to_jiff_to_chrono(date in chrono_date_in_jiff_range()) {
        let jiff_date = date.try_to_jiff().expect("date is within jiff's range");
        prop_assert_eq!(jiff_date.to_chrono(), date);
    }

    #[test]
    fn date_roundtrip_jiff_to_chrono_to_jiff(date in arb::<jiff::civil::Date>()) {
        let chrono_date = date.to_chrono();
        prop_assert_eq!(chrono_date.try_to_jiff().expect("date came from jiff's range"), date);
    }

    #[test]
    fn date_chrono_to_jiff_matches_components(date in chrono_date_in_jiff_range()) {
        let jiff_date = date.try_to_jiff().expect("date is within jiff's range");
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
        prop_assert!(date.try_to_jiff().is_err());
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
        let jiff_datetime = datetime.try_to_jiff().expect("datetime is within jiff's range");
        prop_assert_eq!(jiff_datetime.to_chrono(), datetime);
    }

    #[test]
    fn datetime_roundtrip_jiff_to_chrono_to_jiff(datetime in arb::<jiff::civil::DateTime>()) {
        let chrono_datetime = datetime.to_chrono();
        prop_assert_eq!(
            chrono_datetime.try_to_jiff().expect("datetime came from jiff's range"),
            datetime
        );
    }

    #[test]
    fn datetime_outside_jiff_range_fails_to_convert(
        date in chrono_date_outside_jiff_range(),
        time in arb::<chrono::NaiveTime>(),
    ) {
        let datetime = chrono::NaiveDateTime::new(date, time);
        prop_assert!(datetime.try_to_jiff().is_err());
    }

    #[test]
    fn datetime_chrono_to_jiff_matches_components(
        date in chrono_date_in_jiff_range(),
        time in arb::<chrono::NaiveTime>().prop_filter("not a leap second", |t| t.nanosecond() < 1_000_000_000),
    ) {
        let datetime = chrono::NaiveDateTime::new(date, time);
        let jiff_datetime = datetime.try_to_jiff().expect("datetime is within jiff's range");
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
        let jiff_ts: jiff::Timestamp =
            chrono_dt.try_to_jiff().expect("timestamp is within jiff's range");
        prop_assert_eq!(jiff_ts.to_chrono(), chrono_dt);
    }

    #[test]
    fn timestamp_roundtrip_jiff_to_chrono_to_jiff(jiff_ts in arb::<jiff::Timestamp>()) {
        let chrono_dt = jiff_ts.to_chrono();
        let roundtripped: jiff::Timestamp =
            chrono_dt.try_to_jiff().expect("timestamp came from jiff's range");
        prop_assert_eq!(roundtripped, jiff_ts);
    }

    #[test]
    fn timestamp_outside_jiff_range_fails_to_convert(chrono_dt in chrono_utc_datetime_outside_jiff_range()) {
        let result: Result<jiff::Timestamp, _> = chrono_dt.try_to_jiff();
        prop_assert!(result.is_err());
    }

    #[test]
    fn timestamp_leap_second_normalizes_to_last_nanosecond_of_second(
        chrono_dt in chrono_utc_datetime_leap_second_in_jiff_range()
    ) {
        let jiff_ts: jiff::Timestamp =
            chrono_dt.try_to_jiff().expect("timestamp is within jiff's range");
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
        let jiff_ts: jiff::Timestamp =
            chrono_dt.try_to_jiff().expect("timestamp is within jiff's range");
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
        let jiff_offset: jiff::tz::Offset = chrono_offset.to_jiff();
        prop_assert_eq!(
            jiff_offset.try_to_chrono().expect("offset came from chrono's range"),
            chrono_offset
        );
    }

    #[test]
    fn offset_roundtrip_jiff_to_chrono_to_jiff(jiff_offset in jiff_offset_in_chrono_range()) {
        let chrono_offset = jiff_offset.try_to_chrono().expect("offset is within chrono's range");
        let roundtripped: jiff::tz::Offset = chrono_offset.to_jiff();
        prop_assert_eq!(roundtripped, jiff_offset);
    }

    #[test]
    fn offset_outside_chrono_range_fails_to_convert(jiff_offset in jiff_offset_outside_chrono_range()) {
        prop_assert!(jiff_offset.try_to_chrono().is_err());
    }

    #[test]
    fn offset_chrono_to_jiff_matches_components(chrono_offset in arb::<chrono::FixedOffset>()) {
        let jiff_offset: jiff::tz::Offset = chrono_offset.to_jiff();
        prop_assert_eq!(jiff_offset.seconds(), chrono_offset.local_minus_utc());
    }

    #[test]
    fn offset_jiff_to_chrono_matches_components(jiff_offset in jiff_offset_in_chrono_range()) {
        let chrono_offset = jiff_offset.try_to_chrono().expect("offset is within chrono's range");
        prop_assert_eq!(chrono_offset.local_minus_utc(), jiff_offset.seconds());
    }
}

/* DateTime with fixed offset */

proptest! {
    #[test]
    fn zoned_roundtrip_chrono_to_jiff_to_chrono(chrono_dt in chrono_fixedoffset_datetime_in_jiff_range()) {
        let zoned = chrono_dt.try_to_jiff().expect("value is within jiff's range");
        let back: chrono::DateTime<chrono::FixedOffset> =
            zoned.try_to_chrono().expect("offset is within chrono's range");
        prop_assert_eq!(back, chrono_dt);
    }

    #[test]
    fn zoned_roundtrip_jiff_to_chrono_to_jiff(
        timestamp in arb::<jiff::Timestamp>(),
        offset in jiff_offset_in_chrono_range(),
    ) {
        let zoned = timestamp.to_zoned(jiff::tz::TimeZone::fixed(offset));
        let chrono_dt: chrono::DateTime<chrono::FixedOffset> =
            zoned.try_to_chrono().expect("offset is within chrono's range");
        let roundtripped = chrono_dt.try_to_jiff().expect("value came from jiff's range");
        prop_assert_eq!(roundtripped.timestamp(), zoned.timestamp());
        prop_assert_eq!(roundtripped.offset(), zoned.offset());
    }

    #[test]
    fn zoned_with_offset_outside_chrono_range_fails_to_convert(
        timestamp in arb::<jiff::Timestamp>(),
        offset in jiff_offset_outside_chrono_range(),
    ) {
        let zoned = timestamp.to_zoned(jiff::tz::TimeZone::fixed(offset));
        let result: Result<chrono::DateTime<chrono::FixedOffset>, _> = zoned.try_to_chrono();
        prop_assert!(result.is_err());
    }

    #[test]
    fn zoned_chrono_to_jiff_matches_components(chrono_dt in chrono_fixedoffset_datetime_in_jiff_range()) {
        let zoned = chrono_dt.try_to_jiff().expect("value is within jiff's range");
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
        let chrono_dt: chrono::DateTime<chrono::FixedOffset> =
            zoned.try_to_chrono().expect("offset is within chrono's range");
        let utc = chrono_dt.with_timezone(&chrono::Utc);
        let actual = utc.timestamp() as i128 * 1_000_000_000 + utc.timestamp_subsec_nanos() as i128;
        let expected =
            timestamp.as_second() as i128 * 1_000_000_000 + timestamp.subsec_nanosecond() as i128;
        prop_assert_eq!(actual, expected);
        prop_assert_eq!(chrono_dt.offset().local_minus_utc(), offset.seconds());
    }
}

/* DateTime with UTC */

proptest! {
    #[test]
    fn zoned_utc_roundtrip_chrono_to_jiff_to_chrono(chrono_dt in chrono_utc_datetime_in_jiff_range()) {
        let zoned: jiff::Zoned = chrono_dt.try_to_jiff().expect("value is within jiff's range");
        let back: chrono::DateTime<chrono::Utc> = zoned.to_chrono();
        prop_assert_eq!(back, chrono_dt);
    }

    #[test]
    fn zoned_utc_roundtrip_jiff_to_chrono_to_jiff(timestamp in arb::<jiff::Timestamp>()) {
        let zoned = timestamp.to_zoned(jiff::tz::TimeZone::UTC);
        let chrono_dt: chrono::DateTime<chrono::Utc> = zoned.to_chrono();
        let roundtripped: jiff::Zoned =
            chrono_dt.try_to_jiff().expect("value came from jiff's range");
        prop_assert_eq!(roundtripped.timestamp(), zoned.timestamp());
        prop_assert_eq!(roundtripped.offset(), zoned.offset());
    }

    #[test]
    fn zoned_utc_outside_jiff_range_fails_to_convert(
        chrono_dt in chrono_utc_datetime_outside_jiff_range()
    ) {
        let result: Result<jiff::Zoned, _> = chrono_dt.try_to_jiff();
        prop_assert!(result.is_err());
    }

    #[test]
    fn zoned_utc_chrono_to_jiff_matches_components(chrono_dt in chrono_utc_datetime_in_jiff_range()) {
        let zoned: jiff::Zoned = chrono_dt.try_to_jiff().expect("value is within jiff's range");
        let actual = zoned.timestamp().as_second() as i128 * 1_000_000_000
            + zoned.timestamp().subsec_nanosecond() as i128;
        let expected =
            chrono_dt.timestamp() as i128 * 1_000_000_000 + chrono_dt.timestamp_subsec_nanos() as i128;
        prop_assert_eq!(actual, expected);
        prop_assert_eq!(zoned.offset(), jiff::tz::Offset::UTC);
    }

    #[test]
    fn zoned_utc_jiff_to_chrono_matches_components(timestamp in arb::<jiff::Timestamp>()) {
        let zoned = timestamp.to_zoned(jiff::tz::TimeZone::UTC);
        let chrono_dt: chrono::DateTime<chrono::Utc> = zoned.to_chrono();
        let actual =
            chrono_dt.timestamp() as i128 * 1_000_000_000 + chrono_dt.timestamp_subsec_nanos() as i128;
        let expected =
            timestamp.as_second() as i128 * 1_000_000_000 + timestamp.subsec_nanosecond() as i128;
        prop_assert_eq!(actual, expected);
    }
}

/* Time zone */

proptest! {
    #[test]
    fn timezone_roundtrip_for_shared_iana_names(tz in shared_tz()) {
        let time_zone = tz.try_to_jiff().expect("name is known to jiff");
        let back: chrono_tz::Tz = time_zone.try_to_chrono().expect("name is known to chrono-tz");
        prop_assert_eq!(back, tz);
    }
}

/* Time zone with fixed offset */

proptest! {
    #[test]
    fn timezone_fixedoffset_roundtrip_chrono_to_jiff_to_chrono(chrono_offset in arb::<chrono::FixedOffset>()) {
        let jiff_tz: jiff::tz::TimeZone = chrono_offset.to_jiff();
        let back: chrono::FixedOffset =
            jiff_tz.try_to_chrono().expect("offset came from chrono's range");
        prop_assert_eq!(back, chrono_offset);
    }

    #[test]
    fn timezone_fixedoffset_roundtrip_jiff_to_chrono_to_jiff(tz in jiff_fixed_tz_in_chrono_range()) {
        let offset = tz.to_fixed_offset().expect("time zone is a fixed offset");
        let chrono_offset: chrono::FixedOffset =
            tz.try_to_chrono().expect("offset is within chrono's range");
        let roundtripped: jiff::tz::TimeZone = chrono_offset.to_jiff();
        prop_assert_eq!(roundtripped.to_fixed_offset().unwrap(), offset);
    }

    #[test]
    fn timezone_with_offset_outside_chrono_range_fails_to_convert(
        tz in jiff_fixed_tz_outside_chrono_range()
    ) {
        let result: Result<chrono::FixedOffset, _> = tz.try_to_chrono();
        prop_assert!(result.is_err());
    }

    #[test]
    fn timezone_non_fixed_fails_to_convert_to_fixedoffset(tz in jiff_named_tz_not_fixed()) {
        let result: Result<chrono::FixedOffset, _> = tz.try_to_chrono();
        prop_assert!(result.is_err());
    }
}

/* Time zone with UTC */

#[test]
fn timezone_utc_roundtrip_chrono_to_jiff_to_chrono() {
    let jiff_tz: jiff::tz::TimeZone = chrono::Utc.to_jiff();
    let result: Result<chrono::Utc, _> = jiff_tz.try_to_chrono();
    assert!(result.is_ok());
}

#[test]
fn timezone_utc_roundtrip_jiff_to_chrono_to_jiff() {
    let jiff_tz = jiff::tz::TimeZone::UTC;
    let _: chrono::Utc = jiff_tz.try_to_chrono().expect("UTC is UTC");
    let roundtripped: jiff::tz::TimeZone = chrono::Utc.to_jiff();
    assert_eq!(
        roundtripped.to_fixed_offset().unwrap(),
        jiff::tz::Offset::UTC
    );
}

proptest! {
    #[test]
    fn timezone_non_utc_offset_fails_to_convert_to_utc(tz in jiff_fixed_tz_not_utc()) {
        let result: Result<chrono::Utc, _> = tz.try_to_chrono();
        prop_assert!(result.is_err());
    }

    #[test]
    fn timezone_non_fixed_fails_to_convert_to_utc(tz in jiff_named_tz_not_fixed()) {
        let result: Result<chrono::Utc, _> = tz.try_to_chrono();
        prop_assert!(result.is_err());
    }
}

/* DateTime with IANA time zone */

proptest! {
    #[test]
    fn zoned_named_roundtrip_chrono_to_jiff_to_chrono(chrono_dt in chrono_tz_datetime_in_jiff_range()) {
        let zoned = chrono_dt.try_to_jiff().expect("value is within jiff's range and tz is known to jiff");
        let back: chrono::DateTime<chrono_tz::Tz> =
            zoned.try_to_chrono().expect("tz name is known to chrono-tz");
        prop_assert_eq!(back, chrono_dt);
    }

    #[test]
    fn zoned_named_roundtrip_jiff_to_chrono_to_jiff(
        tz in shared_tz(),
        timestamp in arb::<jiff::Timestamp>(),
    ) {
        let jiff_tz = jiff::tz::TimeZone::get(tz.name()).expect("tz name is known to jiff");
        let zoned = timestamp.to_zoned(jiff_tz);
        let chrono_dt: chrono::DateTime<chrono_tz::Tz> =
            zoned.try_to_chrono().expect("tz name is known to chrono-tz");
        let roundtripped = chrono_dt.try_to_jiff().expect("value came from jiff's range");
        prop_assert_eq!(roundtripped.timestamp(), zoned.timestamp());
        prop_assert_eq!(roundtripped.offset(), zoned.offset());
    }

    #[test]
    fn zoned_named_outside_jiff_range_fails_to_convert(
        chrono_dt in chrono_tz_datetime_outside_jiff_range()
    ) {
        prop_assert!(chrono_dt.try_to_jiff().is_err());
    }

    #[test]
    fn zoned_named_chrono_to_jiff_matches_components(chrono_dt in chrono_tz_datetime_in_jiff_range()) {
        let zoned = chrono_dt.try_to_jiff().expect("value is within jiff's range and tz is known to jiff");
        let utc = chrono_dt.with_timezone(&chrono::Utc);
        let actual = zoned.timestamp().as_second() as i128 * 1_000_000_000
            + zoned.timestamp().subsec_nanosecond() as i128;
        let expected = utc.timestamp() as i128 * 1_000_000_000 + utc.timestamp_subsec_nanos() as i128;
        prop_assert_eq!(actual, expected);
        prop_assert_eq!(zoned.time_zone().iana_name(), Some(chrono_dt.timezone().name()));
    }

    #[test]
    fn zoned_named_jiff_to_chrono_matches_components(
        tz in shared_tz(),
        timestamp in arb::<jiff::Timestamp>(),
    ) {
        let jiff_tz = jiff::tz::TimeZone::get(tz.name()).expect("tz name is known to jiff");
        let zoned = timestamp.to_zoned(jiff_tz);
        let chrono_dt: chrono::DateTime<chrono_tz::Tz> =
            zoned.try_to_chrono().expect("tz name is known to chrono-tz");
        let utc = chrono_dt.with_timezone(&chrono::Utc);
        let actual = utc.timestamp() as i128 * 1_000_000_000 + utc.timestamp_subsec_nanos() as i128;
        let expected =
            timestamp.as_second() as i128 * 1_000_000_000 + timestamp.subsec_nanosecond() as i128;
        prop_assert_eq!(actual, expected);
        prop_assert_eq!(chrono_dt.timezone(), tz);
    }
}

/* Helpers */

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

fn jiff_fixed_tz() -> impl Strategy<Value = jiff::tz::TimeZone> {
    arb::<jiff::tz::TimeZone>().prop_filter("fixed-offset time zone", |tz| tz.iana_name().is_none())
}

fn jiff_named_tz() -> impl Strategy<Value = jiff::tz::TimeZone> {
    arb::<jiff::tz::TimeZone>().prop_filter("named IANA time zone", |tz| tz.iana_name().is_some())
}

fn jiff_fixed_tz_in_chrono_range() -> impl Strategy<Value = jiff::tz::TimeZone> {
    let seconds = chrono_offset_seconds_range();
    jiff_fixed_tz().prop_filter("offset within chrono's range", move |tz| {
        let offset = tz.to_fixed_offset().expect("time zone is a fixed offset");
        seconds.contains(&offset.seconds())
    })
}

fn jiff_fixed_tz_outside_chrono_range() -> impl Strategy<Value = jiff::tz::TimeZone> {
    let seconds = chrono_offset_seconds_range();
    jiff_fixed_tz().prop_filter("offset outside chrono's range", move |tz| {
        let offset = tz.to_fixed_offset().expect("time zone is a fixed offset");
        !seconds.contains(&offset.seconds())
    })
}

fn jiff_fixed_tz_not_utc() -> impl Strategy<Value = jiff::tz::TimeZone> {
    jiff_fixed_tz().prop_filter("offset is not UTC", |tz| {
        tz.to_fixed_offset().expect("time zone is a fixed offset") != jiff::tz::Offset::UTC
    })
}

fn jiff_named_tz_not_fixed() -> impl Strategy<Value = jiff::tz::TimeZone> {
    // Most named IANA zones are not fixed offsets (they observe DST at some point in their
    // history), but a few are (e.g. `Etc/UTC`), so filter those out explicitly rather than
    // assuming every named zone qualifies.
    jiff_named_tz().prop_filter("time zone is not a fixed offset", |tz| {
        tz.to_fixed_offset().is_err()
    })
}

fn shared_tz() -> impl Strategy<Value = chrono_tz::Tz> {
    // As in `timezone_roundtrip_for_shared_iana_names` above, only test with names that both
    // `chrono-tz` and `jiff` recognize.
    proptest::sample::select(chrono_tz::TZ_VARIANTS.as_slice())
        .prop_filter("tz name known to jiff", |tz| {
            jiff::tz::TimeZone::get(tz.name()).is_ok()
        })
}

fn chrono_tz_datetime_in_jiff_range() -> impl Strategy<Value = chrono::DateTime<chrono_tz::Tz>> {
    (shared_tz(), chrono_utc_datetime_in_jiff_range()).prop_map(|(tz, utc)| utc.with_timezone(&tz))
}

fn chrono_tz_datetime_outside_jiff_range() -> impl Strategy<Value = chrono::DateTime<chrono_tz::Tz>>
{
    (shared_tz(), chrono_utc_datetime_outside_jiff_range())
        .prop_map(|(tz, utc)| utc.with_timezone(&tz))
}
